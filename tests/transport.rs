use scratch_server::{serve_connection, ServerState};
use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    sync::atomic::Ordering,
};
use transport_core::{response, Action, Event, Machine, Reason, Status};

enum Input {
    Bytes(Vec<u8>),
    Timeout,
    Eof,
    Interrupted,
}
struct Script {
    input: VecDeque<Input>,
    output: Vec<u8>,
    reads: usize,
}
impl Read for Script {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        self.reads += 1;
        match self.input.pop_front().expect("unexpected shell read") {
            Input::Bytes(bytes) => {
                let n = bytes.len().min(out.len());
                out[..n].copy_from_slice(&bytes[..n]);
                if n < bytes.len() {
                    self.input.push_front(Input::Bytes(bytes[n..].to_vec()));
                }
                Ok(n)
            }
            Input::Timeout => Err(io::ErrorKind::TimedOut.into()),
            Input::Interrupted => Err(io::ErrorKind::Interrupted.into()),
            Input::Eof => Ok(0),
        }
    }
}
impl Write for Script {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.output.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn exercise(input: Vec<Input>, status: Option<&[u8]>, calls: usize) -> Script {
    let mut wire = Script {
        input: input.into(),
        output: Vec::new(),
        reads: 0,
    };
    let state = ServerState::default();
    serve_connection(&mut wire, &state).unwrap();
    assert_eq!(state.handler_calls.load(Ordering::Relaxed), calls);
    if let Some(status) = status {
        assert!(wire.output.starts_with(status));
    } else {
        assert!(wire.output.is_empty());
    }
    wire
}
#[test]
fn shell_obeys_event_sequences_and_stops_after_terminal_action() {
    let valid = b"GET /health HTTP/1.1\r\n\r\n";
    let ok = Some(b"HTTP/1.1 200 OK\r\n".as_slice());
    let bad = Some(b"HTTP/1.1 400 Bad Request\r\n".as_slice());
    let script = exercise(
        vec![
            Input::Interrupted,
            Input::Bytes(valid[..5].to_vec()),
            Input::Bytes(valid[5..].to_vec()),
            Input::Timeout,
        ],
        ok,
        1,
    );
    assert_eq!(script.input.len(), 1);
    exercise(vec![Input::Timeout, Input::Bytes(valid.to_vec())], None, 0);
    exercise(vec![Input::Bytes(vec![b'G']), Input::Timeout], bad, 0);
    exercise(vec![Input::Bytes(vec![b'G']), Input::Eof], bad, 0);
    exercise(vec![Input::Eof], bad, 0);
    exercise(
        vec![Input::Bytes(valid.iter().copied().chain([b'x']).collect())],
        bad,
        0,
    );
    let script = exercise(
        (0..4097)
            .map(|_| Input::Bytes(vec![b'G']))
            .chain([Input::Timeout])
            .collect(),
        bad,
        0,
    );
    assert_eq!(script.reads, 4097);
    assert_eq!(script.input.len(), 1);
}
#[test]
fn core_classifies_events_and_absorbs_every_terminal_state() {
    for timeout in [false, true] {
        for partial in [false, true] {
            let mut m = Machine::new();
            if partial {
                assert!(matches!(m.step(Event::Bytes(b"G")), Action::ReadMore));
            }
            let a = m.step(if timeout {
                Event::ReadTimeout
            } else {
                Event::IdleExpired
            });
            assert!(if partial {
                a == Action::Reject(Reason::Slow)
            } else {
                a == Action::CloseSilently
            });
            for event in [
                Event::Eof,
                Event::Bytes(b"GET / HTTP/1.1\r\n\r\n"),
                Event::ReadTimeout,
                Event::IdleExpired,
            ] {
                assert_eq!(m.step(event), Action::Done);
            }
        }
    }
    for bytes in [b"".as_slice(), b"bad\r\n\r\n", b"GET / HTTP/1.1\r\n\r\nx"] {
        let mut m = Machine::new();
        assert_eq!(m.step(Event::Bytes(bytes)), Action::Reject(Reason::Grammar));
        assert_eq!(m.step(Event::Eof), Action::Done);
    }
    let mut m = Machine::new();
    assert_eq!(m.step(Event::Eof), Action::Reject(Reason::Grammar));
    let mut m = Machine::new();
    assert!(matches!(
        m.step(Event::Bytes(b"GET / HTTP/1.1\r\n\r\n")),
        Action::Dispatch(_)
    ));
    assert_eq!(
        m.step(Event::Bytes(b"GET / HTTP/1.1\r\n\r\n")),
        Action::Done
    );
    let mut m = Machine::new();
    for _ in 0..4096 {
        assert_eq!(m.step(Event::Bytes(b"G")), Action::ReadMore);
    }
    assert_eq!(
        m.step(Event::Bytes(&vec![b'G'; 9000])),
        Action::Reject(Reason::Size)
    );
    assert_eq!(m.bytes().len(), 4097);
    assert!(m.steps() <= 4098 && m.terminal());
    assert_eq!(m.step(Event::IdleExpired), Action::Done);
}
#[test]
fn response_lengths_head_and_arbitrary_body_cannot_change_headers() {
    for n in (0..=4096).chain([4097, 10_000, 100_000]) {
        let body: Vec<u8> = (0..n).map(|i| [0, 13, 10, b':', 255][i % 5]).collect();
        for (status, line) in [
            (Status::Ok, b"HTTP/1.1 200 OK".as_slice()),
            (Status::NotFound, b"HTTP/1.1 404 Not Found"),
            (Status::BadRequest, b"HTTP/1.1 400 Bad Request"),
        ] {
            let get = response(status, &body, false).unwrap();
            let head = response(status, &body, true).unwrap();
            let end = get.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
            assert!(head == get[..end]);
            assert!(get[end..] == body);
            let text = std::str::from_utf8(&head).unwrap();
            let lines: Vec<_> = text.split("\r\n").collect();
            assert!(lines[0].as_bytes() == line);
            assert_eq!(lines.len(), 6);
            assert_eq!(
                lines[1]
                    .strip_prefix("Content-Length: ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                n
            );
            assert_eq!(lines[2], "Content-Type: text/plain");
            assert_eq!(lines[3], "Connection: close");
        }
    }
}

#[test]
fn aead_helpers_match_rustls_and_withhold_failed_plaintext() {
    use transport_core::aead;
    let iv_bytes = [0xa5; 12];
    let iv = rustls::crypto::cipher::Iv::from(iv_bytes);
    for seq in [0, 1, 255, 256, 1 << 32, 1 << 63, u64::MAX] {
        let ours = aead::nonce(&iv_bytes, seq).unwrap();
        assert!(ours == rustls::crypto::cipher::Nonce::new(&iv, seq).0);
        let mut expected = iv_bytes;
        for (i, b) in seq.to_be_bytes().iter().enumerate() {
            expected[i + 4] ^= b;
        }
        assert!(ours == expected);
    }
    assert!(aead::nonce(&[0; 11], 0).is_none());
    assert!(aead::nonce(&[0; 13], 0).is_none());
    for n in [
        0,
        1,
        15,
        16,
        17,
        18,
        4096,
        usize::MAX - 17,
        usize::MAX - 16,
        usize::MAX,
    ] {
        assert_eq!(aead::sealed_len(n), n.checked_add(17));
        assert_eq!(
            aead::plaintext_len(n),
            if n >= 17 { Some(n - 16) } else { None }
        );
    }
    let mut dest = [9; 32];
    assert!(!aead::release(false, &[1; 32], &mut dest));
    assert!(dest == [9; 32]);
    assert!(aead::release(true, &[1; 32], &mut dest));
    assert!(dest == [1; 32]);
}
