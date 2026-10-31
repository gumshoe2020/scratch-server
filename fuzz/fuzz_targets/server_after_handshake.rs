//! Post-handshake records into a production ServerConnection. Each input first completes a
//! real handshake with an in-process rustls client using our provider, then runs ops:
//!   op byte even: raw  — next byte is a length L, then L bytes go to read_tls unframed;
//!   op byte odd:  sealed — next byte is the inner content type, then a big-endian u16
//!                 length and that many payload bytes, sealed with the client's real
//!                 application traffic key (our ChaCha20-Poly1305 encrypter) at the next
//!                 sequence number. Inner type 0 lets the payload carry its own padding.
//! This reaches record decryption, unpadding, alerts, KeyUpdate and post-handshake messages.
//! After a KeyUpdate the harness keeps the old key, so later sealed records fail to decrypt.
#![no_main]
use libfuzzer_sys::fuzz_target;
use rustls::crypto::cipher::{MessageEncrypter, OutboundPlainMessage};
use rustls::{ConnectionTrafficSecrets, ContentType, ProtocolVersion, ServerConnection};
use rustls::{ClientConfig, SupportedCipherSuite};
use scratch_server_fuzz::{client, client_config, drain_server, handshake, server_config};
use std::sync::{Arc, OnceLock};

const MAX_SEALED: usize = 16384 + 256;

fn config() -> &'static Arc<ClientConfig> {
    static CLIENT: OnceLock<Arc<ClientConfig>> = OnceLock::new();
    CLIENT.get_or_init(|| client_config(true))
}

fn take<'a>(data: &mut &'a [u8], n: usize) -> &'a [u8] {
    let n = n.min(data.len());
    let (head, rest) = data.split_at(n);
    *data = rest;
    head
}

/// Feed a whole buffer, processing as rustls accepts it; false once the server errors.
fn feed(server: &mut ServerConnection, mut wire: &[u8]) -> bool {
    while !wire.is_empty() {
        match server.read_tls(&mut wire) {
            Ok(0) | Err(_) => return false,
            Ok(_) => {}
        }
        let result = server.process_new_packets();
        if let (Err(e), true) = (&result, std::env::var_os("STAGE17_DEBUG").is_some()) {
            eprintln!("server error: {e:?}");
        }
        let ok = result.is_ok();
        drain_server(server);
        if !ok {
            return false;
        }
    }
    true
}

fuzz_target!(|data: &[u8]| {
    let mut server = ServerConnection::new(server_config()).unwrap();
    let mut peer = client(config());
    if !handshake(&mut server, &mut peer) {
        panic!("in-memory handshake with the production config failed");
    }
    drain_server(&mut server);
    let (mut seq, secrets) = peer.dangerous_extract_secrets().expect("secrets").tx;
    let ConnectionTrafficSecrets::Chacha20Poly1305 { key, iv } = secrets else {
        panic!("unexpected suite");
    };
    // Without the tls12 feature, Tls13 is the only SupportedCipherSuite variant.
    let SupportedCipherSuite::Tls13(suite) = scratch_server::tls::provider().cipher_suites[0];
    let mut sealer: Box<dyn MessageEncrypter> = suite.aead_alg.encrypter(key, iv);

    let mut data = data;
    while let Some((&op, rest)) = data.split_first() {
        data = rest;
        let wire = if op & 1 == 0 {
            let len = take(&mut data, 1).first().copied().unwrap_or(0);
            take(&mut data, usize::from(len)).to_vec()
        } else {
            let head = take(&mut data, 3);
            if head.len() < 3 {
                break;
            }
            let len = usize::from(u16::from_be_bytes([head[1], head[2]])).min(MAX_SEALED);
            let payload = take(&mut data, len);
            let msg = OutboundPlainMessage {
                typ: ContentType::from(head[0]),
                version: ProtocolVersion::TLSv1_2,
                payload: payload.into(),
            };
            let Ok(sealed) = sealer.encrypt(msg, seq) else {
                break;
            };
            seq += 1;
            sealed.encode()
        };
        if !feed(&mut server, &wire) {
            break;
        }
    }
});
