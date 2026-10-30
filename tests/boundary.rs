use scratch_server::{serve_connection, ServerState};
use std::{
    io::{self, Read, Write},
    sync::atomic::Ordering,
};

struct Wire {
    input: Vec<u8>,
    position: usize,
    chunk: usize,
    output: Vec<u8>,
}
impl Read for Wire {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = (self.input.len() - self.position)
            .min(buf.len())
            .min(self.chunk);
        buf[..n].copy_from_slice(&self.input[self.position..self.position + n]);
        self.position += n;
        Ok(n)
    }
}
impl Write for Wire {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.output.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn exchange(input: &[u8], chunk: usize, state: &ServerState) -> Vec<u8> {
    let mut wire = Wire {
        input: input.to_vec(),
        position: 0,
        chunk,
        output: vec![],
    };
    serve_connection(&mut wire, state).unwrap();
    wire.output
}
fn malformed() -> Vec<Vec<u8>> {
    include_str!("malformed.hex")
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|b| u8::from_str_radix(b, 16).unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn corpus_never_reaches_handler() {
    let mut corpus = malformed();
    corpus.push(Vec::new());
    assert_eq!(corpus.len(), 48);
    for (i, input) in corpus.iter().enumerate() {
        for chunk in [1, 7, 1024] {
            let state = ServerState::with_policy(transport_core::http::Policy {
                unknown_drop: false,
                guarded_drop: 0,
                log_limit: 60,
            });
            let reply = exchange(input, chunk, &state);
            assert!(
                reply.starts_with(b"HTTP/1.1 400 Bad Request\r\n"),
                "case {i}, chunk {chunk}"
            );
            assert_eq!(
                state.handler_calls.load(Ordering::Relaxed),
                0,
                "case {i}, chunk {chunk}"
            );
        }
    }
}

#[test]
fn valid_requests_survive_fragmentation() {
    for (request, status, body) in [
        (
            b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n".as_slice(),
            "200 OK",
            "Hello from a FROM-scratch container.",
        ),
        (b"GET /health HTTP/1.0\r\n\r\n", "200 OK", "ok"),
        (
            b"GET /missing HTTP/1.1\r\n\r\n",
            "404 Not Found",
            "not found",
        ),
        (
            b"HEAD /health HTTP/1.1\r\nAccept: */*\r\n\r\n",
            "200 OK",
            "",
        ),
    ] {
        for chunk in [1, 3, 1024] {
            let state = ServerState::with_policy(transport_core::http::Policy {
                unknown_drop: false,
                guarded_drop: 0,
                log_limit: 60,
            });
            let reply = String::from_utf8(exchange(request, chunk, &state)).unwrap();
            assert!(reply.starts_with(&format!("HTTP/1.1 {status}\r\n")));
            let received = reply.split_once("\r\n\r\n").unwrap().1;
            assert!(received.starts_with(body));
            if request.starts_with(b"HEAD") {
                assert!(received.is_empty());
                assert!(reply.contains("Content-Length: 2\r\n"));
            }
            assert_eq!(state.handler_calls.load(Ordering::Relaxed), 1);
        }
    }
}

#[test]
fn tcp_routes_work() {
    use std::net::{TcpListener, TcpStream};
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let worker = std::thread::spawn(move || {
        for _ in 0..2 {
            let (mut s, _) = listener.accept().unwrap();
            s.set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .unwrap();
            serve_connection(
                &mut s,
                &ServerState::with_policy(transport_core::http::Policy {
                    unknown_drop: false,
                    guarded_drop: 0,
                    log_limit: 60,
                }),
            )
            .unwrap();
        }
    });
    for path in ["/", "/health"] {
        let mut s = TcpStream::connect(addr).unwrap();
        s.set_read_timeout(Some(std::time::Duration::from_secs(2)))
            .unwrap();
        write!(s, "GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut reply = String::new();
        s.read_to_string(&mut reply).unwrap();
        assert!(reply.starts_with("HTTP/1.1 200 OK\r\n"));
    }
    worker.join().unwrap();
}

#[test]
fn coalesced_body_and_pipeline_rejected() {
    // If appended bytes have already arrived in the same read, parse must reject
    // them. Bytes arriving only after a complete request are discarded on close.
    for suffix in [b"body".as_slice(), b"GET / HTTP/1.1\r\n\r\n"] {
        let mut input = b"GET / HTTP/1.1\r\n\r\n".to_vec();
        input.extend_from_slice(suffix);
        let state = ServerState::with_policy(transport_core::http::Policy {
            unknown_drop: false,
            guarded_drop: 0,
            log_limit: 60,
        });
        assert!(exchange(&input, 1024, &state).starts_with(b"HTTP/1.1 400"));
        assert_eq!(state.handler_calls.load(Ordering::Relaxed), 0);
    }
}

#[test]
fn captured_safari_requests() {
    for (bytes, status) in [
        (include_bytes!("safari-root.http").as_slice(), "200 OK"),
        (
            include_bytes!("safari-favicon.http").as_slice(),
            "404 Not Found",
        ),
    ] {
        let _request = http_subset::parse(bytes).expect("captured Safari request parses");
        for chunk in [usize::MAX, 1, 7, 1024] {
            let state = ServerState::with_policy(transport_core::http::Policy {
                unknown_drop: false,
                guarded_drop: 0,
                log_limit: 60,
            });
            let response = String::from_utf8(exchange(bytes, chunk, &state)).unwrap();
            assert!(response.starts_with(&format!("HTTP/1.1 {status}\r\n")));
            assert!(response.contains("\r\nConnection: close\r\n"));
            assert_eq!(state.handler_calls.load(Ordering::Relaxed), 1);
        }
    }
}

#[test]
fn whitelist_rejection_boundaries() {
    let mut lines = vec![
        "Accept-Languages: x".to_owned(),
        "Refere: x".to_owned(),
        "Cookie: x".to_owned(),
        "Sec-Fetch-Destination: x".to_owned(),
        "Sec-Fetch-: x".to_owned(),
        "Sec-Fetch-Users: x".to_owned(),
        "Sec-Fetch-Use: x".to_owned(),
        "sec-ch-ua-mobil: x".to_owned(),
    ];
    for name in [
        "Accept-Language",
        "Accept-Encoding",
        "Upgrade-Insecure-Requests",
        "Priority",
        "Referer",
        "Sec-Fetch-Dest",
        "Sec-Fetch-Site",
        "Sec-Fetch-Mode",
        "Sec-Fetch-User",
        "Cache-Control",
        "Sec-GPC",
        "Sec-CH-UA",
        "Sec-CH-UA-Mobile",
        "Sec-CH-UA-Platform",
        "Pragma",
    ] {
        lines.push(format!("{name}: "));
        lines.push(format!("{name}: {}", "x".repeat(257)));
        // The maximum value remains valid for each new name.
        assert!(http_subset::parse(
            format!("GET / HTTP/1.1\r\n{name}: {}\r\n\r\n", "x".repeat(256)).as_bytes()
        )
        .is_some());
    }

    for name in [
        "Content-Length",
        "Transfer-Encoding",
        "Cookie",
        "Sec-Fetch-Users",
        "Sec-Fetch-",
        "sec-ch-ua-mobil",
        "Pragmas",
        "Pragm",
    ] {
        for form in [
            name.to_ascii_lowercase(),
            name.to_ascii_uppercase(),
            mixed_case(name),
        ] {
            lines.push(format!("{form}: x"));
        }
    }
    for line in lines {
        let request = format!("GET / HTTP/1.1\r\n{line}\r\n\r\n");
        assert!(http_subset::parse_policy(
            request.as_bytes(),
            &http_subset::Policy {
                unknown_drop: false,
                guarded_drop: 0,
                log_limit: 60
            }
        )
        .is_none());
        for chunk in [usize::MAX, 1, 7, 1024] {
            let state = ServerState::with_policy(transport_core::http::Policy {
                unknown_drop: false,
                guarded_drop: 0,
                log_limit: 60,
            });
            assert!(exchange(request.as_bytes(), chunk, &state)
                .starts_with(b"HTTP/1.1 400 Bad Request\r\n"));
            assert_eq!(state.handler_calls.load(Ordering::Relaxed), 0);
        }
    }
}

fn mixed_case(name: &str) -> String {
    name.bytes()
        .enumerate()
        .map(|(i, b)| {
            if i % 2 == 0 {
                b.to_ascii_lowercase()
            } else {
                b.to_ascii_uppercase()
            }
        })
        .map(char::from)
        .collect()
}

#[test]
fn case_insensitive_names_and_header_count() {
    for name in NAMES {
        for form in [
            name.to_ascii_lowercase(),
            name.to_ascii_uppercase(),
            mixed_case(name),
        ] {
            let wire = format!("GET / HTTP/1.1\r\n{form}: x\r\n\r\n");
            for chunk in [usize::MAX, 1, 7, 1024] {
                let state = ServerState::with_policy(transport_core::http::Policy {
                    unknown_drop: false,
                    guarded_drop: 0,
                    log_limit: 60,
                });
                assert!(exchange(wire.as_bytes(), chunk, &state).starts_with(b"HTTP/1.1 200"));
                assert_eq!(state.handler_calls.load(Ordering::Relaxed), 1);
            }
        }
    }
    for count in [24, 25] {
        let wire = format!("GET / HTTP/1.1\r\n{}\r\n", "aCcEpT: x\r\n".repeat(count));
        for chunk in [usize::MAX, 1, 7, 1024] {
            let state = ServerState::with_policy(transport_core::http::Policy {
                unknown_drop: false,
                guarded_drop: 0,
                log_limit: 60,
            });
            let response = exchange(wire.as_bytes(), chunk, &state);
            assert!(response.starts_with(if count == 24 {
                b"HTTP/1.1 200"
            } else {
                b"HTTP/1.1 400"
            }));
            assert_eq!(
                state.handler_calls.load(Ordering::Relaxed),
                if count == 24 { 1 } else { 0 }
            );
        }
    }
    // Folding must not extend to method/version or non-ASCII name bytes.
    for wire in [
        b"get / HTTP/1.1\r\n\r\n".as_slice(),
        b"GET / http/1.1\r\n\r\n",
        b"GET / HTTP/1.1\r\nHo\xd3t: x\r\n\r\n",
    ] {
        assert!(http_subset::parse(wire).is_none());
    }
}
const NAMES: &[&str] = &[
    "Host",
    "Accept",
    "Connection",
    "User-Agent",
    "Accept-Language",
    "Accept-Encoding",
    "Upgrade-Insecure-Requests",
    "Priority",
    "Referer",
    "Sec-Fetch-Dest",
    "Sec-Fetch-Site",
    "Sec-Fetch-Mode",
    "Sec-Fetch-User",
    "Cache-Control",
    "Sec-GPC",
    "Sec-CH-UA",
    "Sec-CH-UA-Mobile",
    "Sec-CH-UA-Platform",
    "Pragma",
];
