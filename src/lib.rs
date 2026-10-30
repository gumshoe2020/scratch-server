//! Transport and application code. HTTP byte interpretation lives in http-subset.
pub mod tls;
pub mod transport;
use std::{
    io::{self, Read, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};
use transport_core::{
    http::{classify, drop_header, Policy},
    response, Action, Event, Machine, Reason, Status,
};

pub struct ServerState {
    pub handler_calls: AtomicUsize,
    pub policy: Policy,
    unknown_log: Mutex<UnknownLog>,
}
impl Default for ServerState {
    fn default() -> Self {
        Self::with_policy(Policy::builtin())
    }
}
impl ServerState {
    pub fn with_policy(policy: Policy) -> Self {
        Self {
            handler_calls: AtomicUsize::new(0),
            policy,
            unknown_log: Mutex::new(UnknownLog::default()),
        }
    }
}
#[derive(Default)]
struct UnknownLog {
    names: Vec<(String, Instant)>,
}
impl UnknownLog {
    fn should_log(&mut self, name: &[u8], limit: u16, now: Instant) -> bool {
        self.names
            .retain(|(_, time)| now.duration_since(*time) < Duration::from_secs(60));
        if self.names.len() >= usize::from(limit).min(64)
            || self
                .names
                .iter()
                .any(|(old, _)| old.as_bytes().eq_ignore_ascii_case(name))
        {
            return false;
        }
        self.names
            .push((String::from_utf8_lossy(name).to_ascii_lowercase(), now));
        true
    }
}
fn sanitized(name: &[u8]) -> String {
    name.iter()
        .copied()
        .filter(|b| b.is_ascii_alphanumeric() || *b == b'-')
        .take(64)
        .map(char::from)
        .collect()
}
// Name-only diagnostic walk. Its output never controls parser acceptance.
fn header_names(bytes: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut rest = bytes;
    let mut first = true;
    std::iter::from_fn(move || loop {
        let end = rest.windows(2).position(|w| w == b"\r\n")?;
        let line = &rest[..end];
        rest = &rest[end + 2..];
        if first {
            first = false;
            continue;
        }
        if line.is_empty() {
            rest = &[];
            return None;
        }
        if let Some(colon) = line.iter().position(|b| *b == b':') {
            return Some(&line[..colon]);
        }
    })
}
fn log_unknown(bytes: &[u8], status: u16, state: &ServerState) {
    let Ok(mut limiter) = state.unknown_log.lock() else {
        return;
    };
    for name in header_names(bytes) {
        if classify(name) == 44 && limiter.should_log(name, state.policy.log_limit, Instant::now()) {
            let _ = writeln!(
                io::stdout().lock(),
                "status={status} note=unknown_header name={}",
                sanitized(name)
            );
        }
    }
}

fn reply(stream: &mut impl Write, status: Status, body: &[u8], head: bool) -> io::Result<()> {
    let bytes = response(status, body, head)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "response too large"))?;
    stream.write_all(&bytes)?;
    stream.flush()
}

fn dispatch(
    stream: &mut impl Write,
    path: &[u8],
    head: bool,
    state: &ServerState,
) -> io::Result<()> {
    state.handler_calls.fetch_add(1, Ordering::Relaxed);
    match path {
        b"/health" => reply(stream, Status::Ok, b"ok", head),
        b"/" => {
            let missing: Vec<&str> = [
                "/bin/sh",
                "/bin/bash",
                "/usr/bin/python3",
                "/usr/bin/curl",
                "/bin/ls",
            ]
            .into_iter()
            .filter(|p| !std::path::Path::new(p).exists())
            .collect();
            let body = format!(
                "Hello from a FROM-scratch container.\nNot present in this image: {}\n",
                missing.join(", ")
            );
            reply(stream, Status::Ok, body.as_bytes(), head)
        }
        _ => reply(stream, Status::NotFound, b"not found\n", head),
    }
}

// Diagnostics are trusted code: they never determine grammar acceptance.
fn rejection_line(bytes: &[u8], policy: &Policy) -> String {
    let mut host = false;
    let mut connection = false;
    let mut count = 0;
    for name in header_names(bytes) {
        count += 1;
        if count > 24 {
            return "status=400 reason=too_many_headers".to_owned();
        }
        let class = classify(name);
        let reason = if (19..28).contains(&class) {
            Some("hard_reject")
        } else if (class == 0 && host) || (class == 2 && connection) {
            Some("duplicate")
        } else if !drop_header(policy, class) {
            Some(if class < 44 {
                "guarded"
            } else {
                "unknown_header"
            })
        } else {
            None
        };
        host |= class == 0;
        connection |= class == 2;
        if let Some(reason) = reason {
            return format!("status=400 reason={reason} name={}", sanitized(name));
        }
    }
    "status=400 reason=grammar".to_owned()
}

fn reject(
    stream: &mut impl Write,
    bytes: &[u8],
    reason: Option<&str>,
    state: &ServerState,
) -> io::Result<()> {
    // Ignore stdout failures so diagnostics cannot prevent the rejection response.
    let line = reason.map_or_else(
        || rejection_line(bytes, &state.policy),
        |r| format!("status=400 reason={r}"),
    );
    let _ = writeln!(io::stdout().lock(), "{line}");
    reply(stream, Status::BadRequest, b"bad request\n", false)
}

/// Every read is submitted to parse before framing or dispatch inspects those bytes.
/// One request per connection. A successful parse must consume the entire read buffer.
/// Later bytes arriving after dispatch are discarded when the connection closes.
pub fn serve_connection(stream: &mut (impl Read + Write), state: &ServerState) -> io::Result<()> {
    serve_with_started(stream, state, || {})
}

pub(crate) fn serve_with_started(
    stream: &mut (impl Read + Write),
    state: &ServerState,
    mut started: impl FnMut(),
) -> io::Result<()> {
    let mut machine = Machine::with_policy(state.policy);
    let mut chunk = [0u8; 1024];
    loop {
        let event = match stream.read(&mut chunk) {
            Ok(0) => Event::Eof,
            Ok(count) => {
                started();
                Event::Bytes(&chunk[..count])
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                ) =>
            {
                Event::ReadTimeout
            }
            Err(e) => return Err(e),
        };
        match machine.step(event) {
            Action::ReadMore => continue,
            Action::Dispatch(request) => {
                let path = &machine.bytes()[request.path_start..request.path_end];
                let status = if path == b"/" || path == b"/health" {
                    200
                } else {
                    404
                };
                log_unknown(machine.bytes(), status, state);
                return dispatch(stream, path, request.head, state);
            }
            Action::Reject(reason) => {
                return reject(
                    stream,
                    machine.bytes(),
                    match reason {
                        Reason::Grammar => None,
                        Reason::Size => Some("size"),
                        Reason::Slow => Some("slow"),
                    },
                    state,
                )
            }
            Action::CloseSilently => {
                let _ = writeln!(io::stdout().lock(), "closed reason=idle");
                return Ok(());
            }
            Action::Done => return Ok(()),
        }
    }
}

#[cfg(test)]
mod policy_tests {
    use super::*;
    #[test]
    fn unknown_log_is_bounded_in_every_sliding_minute() {
        let now = Instant::now();
        for limit in [0, 1, 60, 600] {
            let mut log = UnknownLog::default();
            let mut lines = 0;
            for i in 0..1000 {
                lines += usize::from(log.should_log(format!("X-{}", i % 100).as_bytes(), limit, now));
                assert!(log.names.len() <= 64);
            }
            assert_eq!(lines, usize::from(limit).min(64));
            assert!(!log.should_log(b"x-0", limit, now + Duration::from_secs(59)));
            assert_eq!(
                log.should_log(b"X-0", limit, now + Duration::from_secs(60)),
                limit > 0
            );
            assert!(log.names.len() <= 64);
        }
    }
    #[test]
    fn diagnostic_classes_and_privacy() {
        let p = Policy::builtin();
        for (wire, expected) in [
            (
                b"GET / HTTP/1.1\r\nCookie: synthetic\r\n\r\n".as_slice(),
                "status=400 reason=guarded name=Cookie",
            ),
            (
                b"GET / HTTP/1.1\r\ncOnTeNt-LeNgTh: synthetic\r\n\r\n",
                "status=400 reason=hard_reject name=cOnTeNt-LeNgTh",
            ),
            (
                b"GET / HTTP/1.1\r\nHost: synthetic\r\nhost: synthetic\r\n\r\n",
                "status=400 reason=duplicate name=host",
            ),
            (
                b"GET / HTTP/1.1\r\nHost: synthetic\nCookie: synthetic\r\n\r\n",
                "status=400 reason=grammar",
            ),
        ] {
            assert_eq!(rejection_line(wire, &p), expected);
        }
        let strict = Policy {
            unknown_drop: false,
            guarded_drop: 0,
            log_limit: 60,
        };
        assert_eq!(
            rejection_line(b"GET / HTTP/1.1\r\nDNT: synthetic\r\n\r\n", &strict),
            "status=400 reason=unknown_header name=DNT"
        );
        let wire = format!(
            "GET / HTTP/1.1\r\n{}\r\n",
            "Accept: synthetic\r\n".repeat(25)
        );
        assert_eq!(
            rejection_line(wire.as_bytes(), &p),
            "status=400 reason=too_many_headers"
        );
        assert_eq!(sanitized(b"A!_@-"), "A-");
        assert_eq!(sanitized(&[b'A'; 65]).len(), 64);
    }
}
