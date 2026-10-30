//! Executable intent checks complement the ghost lemmas; request bytes never appear in failures.
use scratch_server::{serve_connection, ServerState};
use std::io::{Cursor, Read, Write};
use transport_core::http::{parse_config, parse_policy, Policy};

#[test]
fn shipped_config_intent() {
    let policy = parse_config(include_bytes!("../config/header-policy.conf")).unwrap();
    assert!(
        policy == Policy::builtin(),
        "shipped policy differs from builtin"
    );
    assert!(policy.unknown_drop && policy.guarded_drop == 0 && policy.log_limit == 60);
    let strict = parse_config(include_bytes!("../config/header-policy-strict.conf")).unwrap();
    assert!(strict.is_strict());
    let guarded = Policy {
        unknown_drop: false,
        guarded_drop: 1,
        log_limit: 60,
    };
    assert!(!guarded.is_strict());
    assert!(parse_policy(b"GET / HTTP/1.1\r\nCookie: x\r\n\r\n", &guarded).is_some());
}

struct Wire {
    input: Cursor<Vec<u8>>,
    output: Vec<u8>,
    chunk: usize,
}
impl Read for Wire {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        let size = bytes.len().min(self.chunk);
        self.input.read(&mut bytes[..size])
    }
}
impl Write for Wire {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.output.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn traversal_paths_are_accepted_and_get_404() {
    for path in [
        "/../x",
        "/..",
        "/../../health",
        "/health/..",
        "/a/../",
        "/.../x",
    ] {
        for method in ["GET", "HEAD"] {
            let bytes = format!("{method} {path} HTTP/1.1\r\n\r\n").into_bytes();
            for unknown_drop in [false, true] {
                let policy = Policy {
                    unknown_drop,
                    guarded_drop: 0,
                    log_limit: 60,
                };
                assert!(parse_policy(&bytes, &policy).is_some(), "traversal grammar");
                for chunk in [1, 7, 1024] {
                    let mut wire = Wire {
                        input: Cursor::new(bytes.clone()),
                        output: vec![],
                        chunk,
                    };
                    serve_connection(&mut wire, &ServerState::with_policy(policy)).unwrap();
                    assert!(
                        wire.output.starts_with(b"HTTP/1.1 404 Not Found\r\n"),
                        "traversal route"
                    );
                }
            }
        }
    }
}

#[test]
fn attack_intent() {
    let mut cases: Vec<Vec<u8>> = vec![
        b"GET / HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\r\nExpect: x\r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\r\nAccept: \r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\n\n".to_vec(),
        b"GET / HTTP/1.1\r\nHost: x\n\n".to_vec(),
        b"GET / HTTP/1.1\n\r\n".to_vec(),
        b"get / HTTP/1.1\r\n\r\n".to_vec(),
        b"GET / HTTP/1.2\r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\r\nConnection: x\r\ncOnNeCtIoN: x\r\n\r\n".to_vec(),
        b"GET /% HTTP/1.1\r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\r\nX Y: x\r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\r\n\r\nx".to_vec(),
    ];
    cases.push(format!("GET / HTTP/1.1\r\nAccept: {}\r\n\r\n", "x".repeat(257)).into_bytes());
    cases.push(format!("GET / HTTP/1.1\r\n{}\r\n", "Accept: x\r\n".repeat(25)).into_bytes());
    // All guarded bits enabled: Expect must remain hard even under this policy.
    for policy in [
        Policy::builtin(),
        Policy {
            unknown_drop: true,
            guarded_drop: u16::MAX,
            log_limit: 60,
        },
    ] {
        for (i, bytes) in cases.iter().enumerate() {
            assert!(parse_policy(bytes, &policy).is_none(), "attack witness {i}");
        }
    }
    assert!(parse_policy(b"GET / HTTP/1.1\r\nCookie: x\r\n\r\n", &Policy::builtin()).is_none());
}
