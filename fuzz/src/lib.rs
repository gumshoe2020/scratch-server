//! Shared harness for the stage 17 fuzz targets. Every server configuration comes from
//! the production `scratch_server::tls::load_config` (our provider, one suite, one group,
//! a P-256 certificate). The throwaway certificate and key are made by
//! `scripts/make-lan-cert.py "$TMPDIR/stage17-tls"` and are never stored in the repository.
use rustls::pki_types::{pem::PemObject, CertificateDer, ServerName};
use rustls::{ClientConfig, ClientConnection, ServerConfig, ServerConnection};
use std::{
    io::{self, Read, Write},
    path::PathBuf,
    sync::{Arc, OnceLock},
};

pub fn tls_dir() -> PathBuf {
    std::env::var_os("STAGE17_TLS_DIR")
        .map(PathBuf::from)
        .expect("set STAGE17_TLS_DIR to a throwaway key directory (scripts/make-lan-cert.py)")
}

/// The production server configuration, loaded once per process.
pub fn server_config() -> Arc<ServerConfig> {
    static CONFIG: OnceLock<Arc<ServerConfig>> = OnceLock::new();
    CONFIG
        .get_or_init(|| scratch_server::tls::load_config(&tls_dir()).expect("load test config"))
        .clone()
}

/// An in-process rustls client using our provider and trusting the throwaway certificate.
pub fn client_config(extract_secrets: bool) -> Arc<ClientConfig> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in CertificateDer::pem_file_iter(tls_dir().join("cert.pem")).expect("cert.pem") {
        roots.add(cert.expect("certificate")).expect("trust anchor");
    }
    let mut config =
        ClientConfig::builder_with_provider(Arc::new(scratch_server::tls::provider()))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("TLS 1.3")
            .with_root_certificates(roots)
            .with_no_client_auth();
    config.alpn_protocols = vec![b"http/1.1".to_vec()];
    config.enable_secret_extraction = extract_secrets;
    Arc::new(config)
}

pub fn client(config: &Arc<ClientConfig>) -> ClientConnection {
    ClientConnection::new(config.clone(), ServerName::try_from("localhost").unwrap())
        .expect("client connection")
}

/// Fuzzer-chosen chunk sizes: seed 0 means one chunk, otherwise a xorshift stream of
/// sizes in 1..=seed*32, so the corpus can steer both the bytes and how they arrive.
pub struct Chunks {
    state: u32,
    max: usize,
}
impl Chunks {
    pub fn new(seed: u8) -> Self {
        Chunks {
            state: 0x9e37_79b9 ^ u32::from(seed),
            max: usize::from(seed) * 32,
        }
    }
    pub fn next(&mut self, remaining: usize) -> usize {
        if self.max == 0 {
            return remaining;
        }
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        (1 + self.state as usize % self.max).min(remaining)
    }
}

pub struct Sink;
impl Write for Sink {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Discard everything the server wants to send and any plaintext it has released.
pub fn drain_server(server: &mut ServerConnection) {
    while server.wants_write() {
        if server.write_tls(&mut Sink).is_err() {
            break;
        }
    }
    let mut buf = [0u8; 4096];
    while let Ok(n) = server.reader().read(&mut buf) {
        if n == 0 {
            break;
        }
    }
}

/// Run a full in-memory handshake between a fresh production server and the client.
pub fn handshake(server: &mut ServerConnection, client: &mut ClientConnection) -> bool {
    for _ in 0..16 {
        let mut wire = Vec::new();
        while client.wants_write() {
            client.write_tls(&mut wire).unwrap();
        }
        let mut rd = &wire[..];
        while !rd.is_empty() {
            server.read_tls(&mut rd).unwrap();
            if server.process_new_packets().is_err() {
                return false;
            }
        }
        let mut wire = Vec::new();
        while server.wants_write() {
            server.write_tls(&mut wire).unwrap();
        }
        let mut rd = &wire[..];
        while !rd.is_empty() {
            client.read_tls(&mut rd).unwrap();
            if client.process_new_packets().is_err() {
                return false;
            }
        }
        if !server.is_handshaking() && !client.is_handshaking() && !client.wants_write() {
            return true;
        }
    }
    false
}
