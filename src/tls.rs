//! Bertie record adapter. Clocks and HTTP decisions remain in their existing cores.
use bertie_tls::{server::ServerDB, tls13crypto::*, tls13utils::AppData, Server, TLSkeyscheduler};
use p256::pkcs8::DecodePrivateKey;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    path::Path,
    sync::Arc,
};
use transport_core::hello;
use zeroize::Zeroizing;

pub struct Config {
    certificate: Vec<u8>,
    key: Zeroizing<Vec<u8>>,
}

pub fn load_config(directory: &Path) -> io::Result<Arc<Config>> {
    let certificate = load_certificate(directory)?;
    let pem = Zeroizing::new(std::fs::read_to_string(directory.join("key.pem"))?);
    let key = p256::SecretKey::from_pkcs8_pem(&pem).map_err(|_| invalid())?;
    Ok(Arc::new(Config {
        certificate,
        key: Zeroizing::new(key.to_bytes().to_vec()),
    }))
}

pub fn load_certificate(directory: &Path) -> io::Result<Vec<u8>> {
    let pem = std::fs::read(directory.join("cert.pem"))?;
    let (label, der) = pem_rfc7468::decode_vec(&pem).map_err(|_| invalid())?;
    if label != "CERTIFICATE" {
        return Err(invalid());
    }
    Ok(der)
}

fn invalid() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "TLS protocol failure")
}

fn algorithms(group: u16) -> io::Result<Algorithms> {
    let kem = match group {
        0x11ec => KemScheme::X25519MlKem768,
        29 => KemScheme::X25519,
        23 => KemScheme::Secp256r1,
        _ => return Err(invalid()),
    };
    Ok(Algorithms::new(
        HashAlgorithm::SHA256,
        AeadAlgorithm::Chacha20Poly1305,
        SignatureScheme::EcdsaSecp256r1Sha256,
        kem,
        false,
        false,
    ))
}

pub fn read_record(socket: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut header = [0; 5];
    if socket.read(&mut header[..1])? == 0 {
        return Ok(Vec::new());
    }
    socket.read_exact(&mut header[1..])?;
    let len = u16::from_be_bytes([header[3], header[4]]) as usize;
    if len == 0 || len > hello::RECORD_LIMIT - 5 || header[1] != 3 || !(1..=3).contains(&header[2])
    {
        return Err(invalid());
    }
    let mut record = header.to_vec();
    record.resize(len + 5, 0);
    socket.read_exact(&mut record[5..])?;
    Ok(record)
}

pub struct BertieStream<S> {
    socket: S,
    state: Option<Server>,
    pending: Vec<u8>,
    offset: usize,
}
impl<S: Read + Write> BertieStream<S> {
    pub fn accept(mut socket: S, config: &Config) -> io::Result<Self> {
        let record = read_record(&mut socket)?;
        let choice = hello::choose(&record).ok_or_else(invalid)?;
        // Only bounded numeric identifiers are logged; no peer record contents.
        eprintln!(
            "tls offered_groups={:?} group={}",
            choice.offered_groups, choice.group
        );
        let db = ServerDB::new(
            choice.sni.into(),
            config.certificate.as_slice().into(),
            config.key.as_slice().into(),
            None,
        );
        let mut ks = TLSkeyscheduler {
            keys: HashMap::new(),
        };
        let (sh, sf, state) = Server::accept(
            algorithms(choice.group)?,
            db,
            &record.into(),
            &mut rand::rng(),
            &mut ks,
        )
        .map_err(|_| invalid())?;
        socket.write_all(&sh.declassify())?;
        socket.write_all(&[20, 3, 3, 0, 1, 1])?;
        socket.write_all(&sf.declassify())?;
        socket.flush()?;
        let mut finished = read_record(&mut socket)?;
        if finished == [20, 3, 3, 0, 1, 1] {
            finished = read_record(&mut socket)?;
        }
        let state = state
            .read_handshake(&finished.into(), &mut ks)
            .map_err(|_| invalid())?;
        Ok(Self {
            socket,
            state: Some(state),
            pending: Vec::new(),
            offset: 0,
        })
    }

    pub fn close_notify(mut self) -> io::Result<S> {
        if let Some(state) = self.state.take() {
            let alert = state.close_notify().map_err(|_| invalid())?;
            self.socket.write_all(&alert.declassify())?;
            self.socket.flush()?;
        }
        Ok(self.socket)
    }
}
impl<S: Read + Write> Read for BertieStream<S> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() {
            return Ok(0);
        }
        while self.offset == self.pending.len() {
            let record = read_record(&mut self.socket)?;
            if record.is_empty() {
                return Ok(0);
            }
            let state = self.state.take().ok_or_else(invalid)?;
            let (data, next) = state.read(&record.into()).map_err(|_| invalid())?;
            self.state = Some(next);
            self.pending = data.map(|d| d.into_raw().declassify()).unwrap_or_default();
            if self.pending.len() > 16384 {
                self.state = None;
                return Err(invalid());
            }
            self.offset = 0;
        }
        let n = out.len().min(self.pending.len() - self.offset);
        out[..n].copy_from_slice(&self.pending[self.offset..self.offset + n]);
        self.offset += n;
        Ok(n)
    }
}
impl<S: Read + Write> Write for BertieStream<S> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }
        let len = bytes.len().min(16384);
        let state = self.state.take().ok_or_else(invalid)?;
        let (record, next) = state
            .write(AppData::new(bytes[..len].into()))
            .map_err(|_| invalid())?;
        self.socket.write_all(&record.declassify())?;
        self.state = Some(next);
        Ok(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.socket.flush()
    }
}

// Fixed TLS 1.3 ClientHello: ChaCha20/SHA256, ECDSA P-256, supported
// MLKEM768/X25519/P-256 groups, X25519 and P-256 public basepoint shares.
// Public fixture only: no private key or cryptography is needed for readiness.
const HEALTH_HELLO: &[u8] = &[
    0x16, 0x03, 0x03, 0x00, 0xcb, 0x01, 0x00, 0x00, 0xc7, 0x03, 0x03, 0x00, 0x01, 0x02, 0x03, 0x04,
    0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
    0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x00, 0x00, 0x02, 0x13, 0x03,
    0x01, 0x00, 0x00, 0x9c, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x0c, 0x00, 0x00, 0x09, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x68, 0x6f, 0x73, 0x74, 0x00, 0x0a, 0x00, 0x08, 0x00, 0x06, 0x11, 0xec, 0x00, 0x1d,
    0x00, 0x17, 0x00, 0x0d, 0x00, 0x04, 0x00, 0x02, 0x04, 0x03, 0x00, 0x2b, 0x00, 0x03, 0x02, 0x03,
    0x04, 0x00, 0x33, 0x00, 0x6b, 0x00, 0x69, 0x00, 0x1d, 0x00, 0x20, 0x09, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x00, 0x41, 0x04,
    0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6, 0xe5, 0x63, 0xa4, 0x40, 0xf2,
    0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0, 0xf4, 0xa1, 0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96,
    0x4f, 0xe3, 0x42, 0xe2, 0xfe, 0x1a, 0x7f, 0x9b, 0x8e, 0xe7, 0xeb, 0x4a, 0x7c, 0x0f, 0x9e, 0x16,
    0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31, 0x5e, 0xce, 0xcb, 0xb6, 0x40, 0x68, 0x37, 0xbf, 0x51, 0xf5,
];

/// Unauthenticated loopback readiness: a bounded ServerHello, not an HTTP check.
pub fn healthcheck(port: u16) -> io::Result<bool> {
    use std::{
        net::{Shutdown, TcpStream},
        time::{Duration, Instant},
    };
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut socket =
        TcpStream::connect_timeout(&(([127, 0, 0, 1], port).into()), Duration::from_secs(2))?;
    let result = (|| {
        socket.set_write_timeout(Some(deadline.saturating_duration_since(Instant::now())))?;
        // A short write is a failed probe; never retry past the absolute budget.
        if socket.write(HEALTH_HELLO)? != HEALTH_HELLO.len() {
            return Ok(false);
        }
        let mut header = [0; 5];
        healthcheck_read(&mut socket, &mut header, deadline)?;
        let len = u16::from_be_bytes([header[3], header[4]]) as usize;
        if header[..3] != [0x16, 3, 3] || !(44..=16384).contains(&len) {
            return Ok(false);
        }
        let mut body = [0; 16384];
        healthcheck_read(&mut socket, &mut body[..len], deadline)?;
        let handshake_len = u32::from_be_bytes([0, body[1], body[2], body[3]]) as usize;
        Ok(body[0] == 2 && (40..=len - 4).contains(&handshake_len))
    })();
    let _ = socket.shutdown(Shutdown::Both);
    result
}

fn healthcheck_read(
    socket: &mut std::net::TcpStream,
    mut bytes: &mut [u8],
    deadline: std::time::Instant,
) -> io::Result<()> {
    while !bytes.is_empty() {
        let remaining = deadline
            .checked_duration_since(std::time::Instant::now())
            .ok_or_else(|| io::Error::from(io::ErrorKind::TimedOut))?;
        socket.set_read_timeout(Some(remaining))?;
        let n = socket.read(bytes)?;
        if n == 0 {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        bytes = &mut bytes[n..];
    }
    Ok(())
}

#[cfg(test)]
#[path = "tls_test_provider.rs"]
mod test_provider;

#[cfg(test)]
impl Config {
    fn from_der(certificate: &[u8], key: &[u8]) -> io::Result<Arc<Self>> {
        let key = p256::SecretKey::from_pkcs8_der(key).map_err(|_| invalid())?;
        Ok(Arc::new(Self {
            certificate: certificate.to_vec(),
            key: Zeroizing::new(key.to_bytes().to_vec()),
        }))
    }
}
