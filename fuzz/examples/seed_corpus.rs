//! Seed corpora for the stage 17 fuzz targets. Needs STAGE17_TLS_DIR (throwaway key).
//!   seed_corpus connect HOST:PORT  — rustls client (our provider) does GET / over TCP, so
//!                                    scripts/stage17-capture-hellos.py records its hello;
//!   seed_corpus write              — fuzz/seeds/hellos/*.bin -> fuzz/seeds/<target>/*.
//! Seeds hold only ClientHello bytes and plaintext HTTP/TLS framing chosen here; never keys.
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

fn connect(addr: &str) {
    let config = scratch_server_fuzz::client_config(false);
    let conn = scratch_server_fuzz::client(&config);
    let mut stream = rustls::StreamOwned::new(conn, TcpStream::connect(addr).unwrap());
    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        .unwrap();
    let mut reply = Vec::new();
    let _ = stream.read_to_end(&mut reply);
    println!("rustls client: {} response bytes", reply.len());
}

/// One server_after_handshake op that seals `payload` with inner content type `typ`.
fn sealed(typ: u8, payload: &[u8]) -> Vec<u8> {
    let mut op = vec![1, typ];
    op.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    op.extend_from_slice(payload);
    op
}
fn raw(bytes: &[u8]) -> Vec<u8> {
    let mut op = vec![0, bytes.len() as u8];
    op.extend_from_slice(bytes);
    op
}

fn write_seeds() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds");
    let put = |target: &str, name: &str, bytes: &[u8]| {
        let dir = root.join(target);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), bytes).unwrap();
    };
    let mut hellos: Vec<_> = fs::read_dir(root.join("hellos"))
        .expect("capture hellos first")
        .map(|e| e.unwrap().path())
        .collect();
    hellos.sort();
    let requests: &[(&str, &[u8])] = &[
        ("get-root", b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
        ("get-health", b"GET /health HTTP/1.0\r\nHost: localhost\r\n\r\n"),
        ("head-root", b"HEAD / HTTP/1.1\r\nHost: localhost\r\nUser-Agent: curl/8.7.1\r\nAccept: */*\r\n\r\n"),
        ("unknown-header", b"GET /x HTTP/1.1\r\nHost: localhost\r\nX-Odd: 1\r\n\r\n"),
        ("guarded-header", b"GET / HTTP/1.1\r\nHost: localhost\r\nCookie: a=b\r\n\r\n"),
        ("bad-method", b"BREW / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
        ("partial", b"GET / HTTP/1.1\r\nHost: loc"),
    ];
    for path in &hellos {
        let name = path.file_stem().unwrap().to_str().unwrap();
        let hello = fs::read(path).unwrap();
        // server_first_flight: chunk seed 0 (one read) and 1 (1..32-byte reads).
        for seed in [0u8, 1] {
            let mut input = vec![seed];
            input.extend_from_slice(&hello);
            put("server_first_flight", &format!("{name}-chunk{seed}"), &input);
        }
        // proven_shell_pipeline raw mode, one chunk.
        let mut input = vec![0u8];
        input.extend_from_slice(&hello);
        put("proven_shell_pipeline", &format!("raw-{name}"), &input);
    }
    for (name, request) in requests {
        // Client mode, no close_notify, one chunk; and with close_notify, 1..32-byte chunks.
        for mode in [1u8, 3 | (1 << 2)] {
            let mut input = vec![mode];
            input.extend_from_slice(request);
            put("proven_shell_pipeline", &format!("client-{name}-m{mode}"), &input);
        }
        put("server_after_handshake", &format!("appdata-{name}"), &sealed(23, request));
    }
    let after: &[(&str, Vec<u8>)] = &[
        ("alert-close-notify", sealed(21, &[1, 0])),
        ("alert-fatal", sealed(21, &[2, 40])),
        ("keyupdate-not-requested", sealed(22, &[24, 0, 0, 1, 0])),
        ("keyupdate-requested", sealed(22, &[24, 0, 0, 1, 1])),
        ("keyupdate-then-data", [sealed(22, &[24, 0, 0, 1, 1]), sealed(23, b"GET / HTTP/1.1\r\n\r\n")].concat()),
        ("unexpected-client-hello", sealed(22, &[1, 0, 0, 4, 3, 3, 0, 0])),
        ("unexpected-new-session-ticket", sealed(22, &[4, 0, 0, 0])),
        ("padded-appdata", sealed(0, b"hi\x17\x00\x00\x00")),
        ("all-zero-inner", sealed(0, &[0, 0, 0])),
        ("inner-ccs", sealed(20, &[1])),
        ("empty-appdata", sealed(23, &[])),
        ("raw-ccs", raw(&[20, 3, 3, 0, 1, 1])),
        ("raw-short-appdata", raw(&[23, 3, 3, 0, 4, 1, 2, 3, 4])),
        ("raw-plain-alert", raw(&[21, 3, 3, 0, 2, 1, 0])),
        ("oversized", sealed(23, &vec![b'a'; 16385])),
    ];
    for (name, input) in after {
        put("server_after_handshake", name, input);
    }
    println!("seeds written for {} hellos", hellos.len());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("connect") => connect(&args[2]),
        Some("write") => write_seeds(),
        _ => eprintln!("usage: seed_corpus connect HOST:PORT | write"),
    }
}
