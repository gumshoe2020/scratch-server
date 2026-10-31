//! Arbitrary peer bytes into a fresh production ServerConnection (the ClientHello path).
//! Byte 0 picks the chunking; the rest is the wire stream, fed via read_tls then
//! process_new_packets until an error or the input is exhausted.
#![no_main]
use libfuzzer_sys::fuzz_target;
use rustls::ServerConnection;
use scratch_server_fuzz::{drain_server, server_config, Chunks};

fuzz_target!(|data: &[u8]| {
    let Some((&seed, mut wire)) = data.split_first() else {
        return;
    };
    let mut chunks = Chunks::new(seed);
    let mut server = ServerConnection::new(server_config()).unwrap();
    while !wire.is_empty() {
        let mut chunk = &wire[..chunks.next(wire.len())];
        let offered = chunk.len();
        let read = match server.read_tls(&mut chunk) {
            Ok(n) => n,
            Err(_) => break,
        };
        wire = &wire[offered - chunk.len()..];
        debug_assert_eq!(read, offered - chunk.len());
        let failed = server.process_new_packets().is_err();
        drain_server(&mut server);
        if failed || read == 0 {
            break;
        }
    }
});
