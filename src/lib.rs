//! Transport shell. Post-machine decisions live in transport_core::app.
pub mod tls;
pub mod transport;
use std::{
    io::{self, Read, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    time::Instant,
};
use transport_core::{
    app::{self, Route, Task},
    http::Policy,
    Event, Machine,
};

pub struct ServerState {
    pub handler_calls: AtomicUsize,
    pub policy: Policy,
    unknown_log: Mutex<app::UnknownLog>,
    clock_origin: Instant,
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
            unknown_log: Mutex::new(app::UnknownLog::new()),
            clock_origin: Instant::now(),
        }
    }
}
fn log_unknown(bytes: &[u8], route: Route, state: &ServerState) {
    let Ok(mut limiter) = state.unknown_log.lock() else {
        return;
    };
    let mut pos = 0;
    while let Some((a, b, next)) = app::next_unknown(bytes, pos) {
        let now = Instant::now()
            .duration_since(state.clock_origin)
            .as_millis();
        let now_ms = u64::try_from(now).unwrap_or(u64::MAX);
        if limiter.should_log(&bytes[a..b], state.policy.log_limit, now_ms) {
            let line = app::unknown_line(route, &bytes[a..b]);
            let _ = writeln!(io::stdout().lock(), "{}", String::from_utf8_lossy(&line));
        }
        pos = next;
    }
}
fn reply(stream: &mut impl Write, bytes: Option<Vec<u8>>) -> io::Result<()> {
    let bytes =
        bytes.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "response too large"))?;
    stream.write_all(&bytes)?;
    stream.flush()
}
fn root_body() -> String {
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
    format!(
        "Hello from a FROM-scratch container.\nNot present in this image: {}\n",
        missing.join(", ")
    )
}

/// Submit every read to the proven machine before application handling.
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
        let read = stream.read(&mut chunk);
        let (step, starts) = transport_core::io::classify(read_result(&read));
        if starts {
            started();
        }
        let event = match step {
            transport_core::io::Step::Retry => continue,
            transport_core::io::Step::Eof => Event::Eof,
            transport_core::io::Step::Bytes(count) => Event::Bytes(&chunk[..count]),
            transport_core::io::Step::ReadTimeout => Event::ReadTimeout,
            transport_core::io::Step::Fail => return Err(read.unwrap_err()),
        };
        let plan = app::plan(machine.step(event));
        match &plan.task {
            Task::ReadMore => continue,
            Task::Dispatch(request) => {
                let route = app::route(&machine.bytes()[request.path_start..request.path_end]);
                log_unknown(machine.bytes(), route, state);
                state
                    .handler_calls
                    .fetch_add(plan.handler_increment, Ordering::Relaxed);
                let body = if app::needs_root(route) {
                    root_body()
                } else {
                    String::new()
                };
                return reply(
                    stream,
                    app::task_response(&plan.task, route, body.as_bytes()),
                );
            }
            Task::Reject(reason) => {
                let line = app::rejection_line(machine.bytes(), &state.policy, reason);
                let _ = writeln!(io::stdout().lock(), "{}", String::from_utf8_lossy(&line));
                return reply(stream, app::task_response(&plan.task, Route::NotFound, b""));
            }
            Task::CloseSilently => {
                let _ = writeln!(io::stdout().lock(), "closed reason=idle");
                return Ok(());
            }
            Task::Done => return Ok(()),
        }
    }
}

// Preserve the original error object for the Fail path.
fn read_result(read: &io::Result<usize>) -> transport_core::io::ReadResult {
    use transport_core::io::ReadResult;
    match read {
        Ok(n) => ReadResult::Data(*n),
        Err(e) => match e.kind() {
            io::ErrorKind::Interrupted => ReadResult::Interrupted,
            io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => ReadResult::Timeout,
            _ => ReadResult::Other,
        },
    }
}

// Test adapters preserve the existing test assertions; production uses byte buffers.
#[cfg(test)]
fn sanitized(name: &[u8]) -> String {
    String::from_utf8(app::sanitize(name)).unwrap()
}
#[cfg(test)]
fn header_names(bytes: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut pos = 0;
    std::iter::from_fn(move || {
        let (a, b, next) = app::next_header_name(bytes, pos)?;
        pos = next;
        Some(&bytes[a..b])
    })
}
#[cfg(test)]
fn rejection_line(bytes: &[u8], policy: &Policy) -> String {
    String::from_utf8(app::diagnostic_line(app::diagnose(bytes, policy), bytes)).unwrap()
}

#[cfg(test)]
mod policy_tests {
    use super::*;
    #[test]
    fn unknown_log_is_bounded_in_every_sliding_minute() {
        let now = 0u64;
        for limit in [0, 1, 60, 600] {
            let mut log = app::UnknownLog::new();
            let mut lines = 0;
            for i in 0..1000 {
                lines +=
                    usize::from(log.should_log(format!("X-{}", i % 100).as_bytes(), limit, now));
                assert!(log.entries.len() <= 64);
            }
            assert_eq!(lines, usize::from(limit).min(64));
            assert!(!log.should_log(b"x-0", limit, now + 59_000));
            assert_eq!(log.should_log(b"X-0", limit, now + 60_000), limit > 0);
            assert!(log.entries.len() <= 64);
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

#[cfg(test)]
mod dispatch_oracle;

#[cfg(test)]
mod io_tests;
