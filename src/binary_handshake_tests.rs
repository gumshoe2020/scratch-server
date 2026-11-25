//! Repeated real TLS 1.3 handshakes from a rustls client to an already-running production binary.
//! Ignored by default; driven by `scripts/binary-handshakes.sh`. Environment: M4_PORT, M4_CERT
//! (cert.pem the binary serves, used as the sole trust anchor), M4_N (default 5000).
use super::provider;
use std::{
    collections::BTreeMap,
    net::TcpStream,
    sync::Arc,
    time::Duration,
};

#[test]
#[ignore]
fn binary_rustls_handshakes() {
    let port: u16 = std::env::var("M4_PORT").unwrap().parse().unwrap();
    let cert_path = std::env::var("M4_CERT").unwrap();
    let total: usize = std::env::var("M4_N").map_or(5000, |n| n.parse().unwrap());
    let pem = std::fs::read(cert_path).unwrap();
    let (_, der) = pem_rfc7468::decode_vec(&pem).unwrap();
    let mut roots = rustls::RootCertStore::empty();
    roots.add(der.into()).unwrap();
    let mut client = rustls::ClientConfig::builder_with_provider(Arc::new(provider()))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_root_certificates(roots)
        .with_no_client_auth();
    client.alpn_protocols = vec![b"http/1.1".to_vec()];
    let client = Arc::new(client);
    let (mut failed, mut first_failure) = (0usize, None);
    let mut reasons: BTreeMap<String, usize> = BTreeMap::new();
    for case in 0..total {
        let mut socket = TcpStream::connect(("127.0.0.1", port)).unwrap();
        socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        socket.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut conn =
            rustls::ClientConnection::new(client.clone(), "localhost".try_into().unwrap())
                .unwrap();
        if let Err(e) = conn.complete_io(&mut socket) {
            failed += 1;
            first_failure.get_or_insert(case);
            *reasons.entry(format!("{e:?}")).or_default() += 1;
        }
    }
    println!(
        "M4-RUSTLS handshakes={total} failed={failed} first_failed_case={first_failure:?} reasons={reasons:?}"
    );
    if std::env::var_os("M4_ASSERT_ZERO").is_some() {
        assert_eq!(failed, 0);
    }
}
