//! TLS/provider shell. Nonce, length arithmetic and plaintext release use proven helpers.
use libcrux_secrets::{ClassifyRef, ClassifyRefMut};
use libcrux_sha2::Digest;
use pki_types::pem::PemObject;
use rustls::{
    crypto::{self, cipher::*, hash, tls13::*, CryptoProvider},
    pki_types::{self, alg_id, AlgorithmIdentifier, PrivateKeyDer, SignatureVerificationAlgorithm},
    sign::{Signer, SigningKey},
    CipherSuite, ConnectionTrafficSecrets, ContentType, Error, NamedGroup, ProtocolVersion,
    SignatureAlgorithm, SignatureScheme, SupportedCipherSuite, Tls13CipherSuite,
};
use std::{
    fmt,
    io::{self},
    path::Path,
    sync::Arc,
};
use transport_core::{aead, record};
use zeroize::Zeroizing;

pub fn provider() -> CryptoProvider {
    CryptoProvider {
        cipher_suites: vec![SupportedCipherSuite::Tls13(&SUITE)],
        kx_groups: vec![&X25519],
        signature_verification_algorithms: crypto::WebPkiSupportedAlgorithms {
            all: &[&P256Verify],
            mapping: &[(SignatureScheme::ECDSA_NISTP256_SHA256, &[&P256Verify])],
        },
        secure_random: &OsRandom,
        key_provider: &P256KeyProvider,
    }
}

static SUITE: Tls13CipherSuite = Tls13CipherSuite {
    common: crypto::CipherSuiteCommon {
        suite: CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
        hash_provider: &Sha256,
        confidentiality_limit: u64::MAX,
    },
    hkdf_provider: &Hkdf,
    aead_alg: &ChaCha,
    quic: None,
};

#[derive(Debug)]
struct OsRandom;
impl crypto::SecureRandom for OsRandom {
    fn fill(&self, buf: &mut [u8]) -> Result<(), crypto::GetRandomFailed> {
        getrandom::fill(buf).map_err(|_| crypto::GetRandomFailed)
    }
}
impl rand_core::TryRng for OsRandom {
    type Error = getrandom::Error;
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut b = [0; 4];
        getrandom::fill(&mut b)?;
        Ok(u32::from_ne_bytes(b))
    }
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut b = [0; 8];
        getrandom::fill(&mut b)?;
        Ok(u64::from_ne_bytes(b))
    }
    fn try_fill_bytes(&mut self, b: &mut [u8]) -> Result<(), Self::Error> {
        getrandom::fill(b)
    }
}
impl rand_core::TryCryptoRng for OsRandom {}

struct Sha256;
#[derive(Clone)]
struct HashContext(libcrux_sha2::Sha256);
impl hash::Hash for Sha256 {
    fn start(&self) -> Box<dyn hash::Context> {
        Box::new(HashContext(libcrux_sha2::Sha256::new()))
    }
    fn hash(&self, data: &[u8]) -> hash::Output {
        hash::Output::new(&libcrux_sha2::sha256(data))
    }
    fn output_len(&self) -> usize {
        transport_core::io::HASH_LEN
    }
    fn algorithm(&self) -> hash::HashAlgorithm {
        hash::HashAlgorithm::SHA256
    }
}
impl hash::Context for HashContext {
    fn fork_finish(&self) -> hash::Output {
        let mut out = [0; 32];
        self.0.finish(&mut out);
        hash::Output::new(&out)
    }
    fn fork(&self) -> Box<dyn hash::Context> {
        Box::new(self.clone())
    }
    fn finish(self: Box<Self>) -> hash::Output {
        self.fork_finish()
    }
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

struct Hkdf;
struct Expander(Zeroizing<Vec<u8>>);
impl crypto::tls13::Hkdf for Hkdf {
    fn extract_from_zero_ikm(&self, salt: Option<&[u8]>) -> Box<dyn HkdfExpander> {
        self.extract_from_secret(salt, &[0; 32])
    }
    fn extract_from_secret(&self, salt: Option<&[u8]>, secret: &[u8]) -> Box<dyn HkdfExpander> {
        let mut prk = Zeroizing::new(vec![0; 32]);
        // TLS bounds both inputs far below the libcrux u32 input limit.
        libcrux_hkdf::extract(
            libcrux_hkdf::Algorithm::Sha256,
            prk.classify_ref_mut(),
            salt.unwrap_or(&[0; 32]).classify_ref(),
            secret.classify_ref(),
        )
        .expect("TLS HKDF extract inputs fit u32");
        Box::new(Expander(prk))
    }
    fn expander_for_okm(&self, okm: &OkmBlock) -> Box<dyn HkdfExpander> {
        Box::new(Expander(Zeroizing::new(okm.as_ref().to_vec())))
    }
    fn hmac_sign(&self, key: &OkmBlock, message: &[u8]) -> crypto::hmac::Tag {
        let mut tag = Zeroizing::new([0; 32]);
        libcrux_hmac::hmac_sha2_256(&mut tag, key.as_ref(), message);
        crypto::hmac::Tag::new(tag.as_ref())
    }
}
impl HkdfExpander for Expander {
    fn expand_slice(&self, info: &[&[u8]], output: &mut [u8]) -> Result<(), OutputLengthError> {
        libcrux_hkdf::expand(
            libcrux_hkdf::Algorithm::Sha256,
            output.classify_ref_mut(),
            self.0.classify_ref(),
            &info.concat(),
        )
        .map_err(|_| OutputLengthError)
    }
    fn expand_block(&self, info: &[&[u8]]) -> OkmBlock {
        let mut out = Zeroizing::new([0; 32]);
        self.expand_slice(info, out.as_mut())
            .expect("HKDF hash-sized output");
        OkmBlock::new(out.as_ref())
    }
    fn hash_len(&self) -> usize {
        transport_core::io::HASH_LEN
    }
}

#[derive(Debug)]
struct X25519;
struct Exchange {
    secret: Zeroizing<[u8; 32]>,
    public: [u8; 32],
}
impl crypto::SupportedKxGroup for X25519 {
    fn start(&self) -> Result<Box<dyn crypto::ActiveKeyExchange>, Error> {
        let mut secret = Zeroizing::new([0; 32]);
        getrandom::fill(secret.as_mut()).map_err(|_| crypto::GetRandomFailed)?;
        libcrux_curve25519::clamp(&mut secret);
        let mut public = [0; 32];
        libcrux_curve25519::secret_to_public(&mut public, &secret);
        Ok(Box::new(Exchange { secret, public }))
    }
    fn name(&self) -> NamedGroup {
        NamedGroup::X25519
    }
}
impl crypto::ActiveKeyExchange for Exchange {
    fn complete(self: Box<Self>, peer: &[u8]) -> Result<crypto::SharedSecret, Error> {
        let invalid = || Error::PeerMisbehaved(rustls::PeerMisbehaved::InvalidKeyShare);
        let peer = record::x25519_peer(peer).ok_or_else(invalid)?;
        let mut shared = Zeroizing::new([0; 32]);
        libcrux_curve25519::ecdh(&mut shared, &peer, &self.secret).map_err(|_| invalid())?;
        Ok(crypto::SharedSecret::from(shared.as_slice()))
    }
    fn pub_key(&self) -> &[u8] {
        &self.public
    }
    fn group(&self) -> NamedGroup {
        NamedGroup::X25519
    }
}

struct ChaCha;
struct RecordCipher {
    key: AeadKey,
    iv: Iv,
}
impl Tls13AeadAlgorithm for ChaCha {
    fn encrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageEncrypter> {
        Box::new(RecordCipher { key, iv })
    }
    fn decrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageDecrypter> {
        Box::new(RecordCipher { key, iv })
    }
    fn key_len(&self) -> usize {
        aead::KEY_LEN
    }
    fn extract_keys(
        &self,
        key: AeadKey,
        iv: Iv,
    ) -> Result<ConnectionTrafficSecrets, UnsupportedOperationError> {
        Ok(ConnectionTrafficSecrets::Chacha20Poly1305 { key, iv })
    }
}
impl MessageEncrypter for RecordCipher {
    fn encrypt(
        &mut self,
        msg: OutboundPlainMessage<'_>,
        seq: u64,
    ) -> Result<OutboundOpaqueMessage, Error> {
        let ciphertext_len = aead::sealed_len(msg.payload.len()).ok_or(Error::EncryptError)?;
        let mut payload_bytes = Zeroizing::new(Vec::with_capacity(msg.payload.len()));
        msg.payload.copy_to_vec(&mut payload_bytes);
        let plain = Zeroizing::new(
            record::inner_plaintext(&payload_bytes, u8::from(msg.typ)).ok_or(Error::EncryptError)?,
        );
        let mut ciphertext = vec![0; ciphertext_len];
        let key = self
            .key
            .as_ref()
            .try_into()
            .map_err(|_| Error::EncryptError)?;
        libcrux_chacha20poly1305::encrypt(
            key,
            &plain,
            &mut ciphertext,
            &record::aad(ciphertext_len),
            &aead::nonce(self.iv.as_ref(), seq).ok_or(Error::EncryptError)?,
        )
        .map_err(|_| Error::EncryptError)?;
        let mut payload = PrefixedPayload::with_capacity(ciphertext.len());
        payload.extend_from_slice(&ciphertext);
        Ok(OutboundOpaqueMessage::new(
            ContentType::ApplicationData,
            ProtocolVersion::TLSv1_2,
            payload,
        ))
    }
    fn encrypted_payload_len(&self, len: usize) -> usize {
        record::encrypted_payload_len(len)
    }
}
impl MessageDecrypter for RecordCipher {
    fn decrypt<'a>(
        &mut self,
        mut msg: InboundOpaqueMessage<'a>,
        seq: u64,
    ) -> Result<InboundPlainMessage<'a>, Error> {
        let len = aead::plaintext_len(msg.payload.len()).ok_or(Error::DecryptError)?;
        let mut plain = Zeroizing::new(vec![0; len]);
        let key = self
            .key
            .as_ref()
            .try_into()
            .map_err(|_| Error::DecryptError)?;
        let authenticated = libcrux_chacha20poly1305::decrypt(
            key,
            &mut plain,
            &msg.payload,
            &record::aad(msg.payload.len()),
            &aead::nonce(self.iv.as_ref(), seq).ok_or(Error::DecryptError)?,
        )
        .is_ok();
        // A reported crypto failure preserves rustls' ciphertext buffer exactly.
        if !aead::release(authenticated, &plain, &mut msg.payload[..len]) {
            return Err(Error::DecryptError);
        }
        msg.payload.truncate(len);
        unpad_message(msg)
    }
}

// Conversion and error wiring remain shell; byte decisions are proven in record.
fn unpad_message(mut msg: InboundOpaqueMessage<'_>) -> Result<InboundPlainMessage<'_>, Error> {
    let unpadded = record::unpad(&msg.payload).map_err(|error| match error {
        record::UnpadError::Oversized => Error::PeerSentOversizedRecord,
        record::UnpadError::NoContentType => rustls::PeerMisbehaved::IllegalTlsInnerPlaintext.into(),
        record::UnpadError::ChangeCipherSpec => rustls::PeerMisbehaved::IllegalMiddleboxChangeCipherSpec.into(),
    })?;
    msg.typ = ContentType::from(unpadded.typ);
    msg.version = ProtocolVersion::TLSv1_3;
    Ok(msg.into_plain_message_range(0..unpadded.len))
}

#[derive(Debug)]
struct P256Verify;
impl SignatureVerificationAlgorithm for P256Verify {
    fn public_key_alg_id(&self) -> AlgorithmIdentifier {
        alg_id::ECDSA_P256
    }
    fn signature_alg_id(&self) -> AlgorithmIdentifier {
        alg_id::ECDSA_SHA256
    }
    fn verify_signature(
        &self,
        public: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), pki_types::InvalidSignature> {
        use libcrux_ecdsa::p256::{PublicKey, Signature};
        let invalid = |_| pki_types::InvalidSignature;
        // TLS/webpki use an uncompressed SEC1 point, not arbitrary raw coordinates.
        if !record::p256_point_shape(public) {
            return Err(pki_types::InvalidSignature);
        }
        let public = PublicKey::try_from(public).map_err(invalid)?;
        let signature = Signature::from_der(signature).map_err(invalid)?;
        libcrux_ecdsa::p256::verify(
            libcrux_sha2::Algorithm::Sha256,
            message,
            &signature,
            &public,
        )
        .map_err(invalid)
    }
}

#[derive(Debug)]
struct P256KeyProvider;
struct P256SigningKey {
    scalar: Zeroizing<[u8; 32]>,
    spki: pki_types::SubjectPublicKeyInfoDer<'static>,
}
impl fmt::Debug for P256SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("P256SigningKey")
    }
}
impl crypto::KeyProvider for P256KeyProvider {
    fn load_private_key(&self, key: PrivateKeyDer<'static>) -> Result<Arc<dyn SigningKey>, Error> {
        use pkcs8::der::Decode;
        let invalid = || Error::General("expected a PKCS8 P-256 key".into());
        let PrivateKeyDer::Pkcs8(der) = key else {
            return Err(invalid());
        };
        let info =
            pkcs8::PrivateKeyInfo::from_der(der.secret_pkcs8_der()).map_err(|_| invalid())?;
        let curve = pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.3.1.7");
        if info.algorithm.oid != pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.2.1")
            || info.algorithm.parameters_oid().map_err(|_| invalid())? != curve
        {
            return Err(invalid());
        }
        let ec = sec1::EcPrivateKey::from_der(info.private_key).map_err(|_| invalid())?;
        if ec
            .parameters
            .is_some_and(|p| p.named_curve() != Some(curve))
        {
            return Err(invalid());
        }
        let scalar: [u8; 32] = ec.private_key.try_into().map_err(|_| invalid())?;
        let scalar = Zeroizing::new(scalar);
        let private =
            libcrux_ecdsa::p256::PrivateKey::try_from(scalar.as_slice()).map_err(|_| invalid())?;
        let public = private.public_key().map_err(|_| invalid())?;
        let mut point = vec![4];
        point.extend_from_slice(&public.0);
        let spki = rustls::sign::public_key_to_spki(&alg_id::ECDSA_P256, point);
        Ok(Arc::new(P256SigningKey { scalar, spki }))
    }
}
struct P256Signer(Zeroizing<[u8; 32]>);
impl fmt::Debug for P256Signer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("P256Signer")
    }
}
impl SigningKey for P256SigningKey {
    fn choose_scheme(&self, offered: &[SignatureScheme]) -> Option<Box<dyn Signer>> {
        let offered: Vec<u16> = offered.iter().copied().map(u16::from).collect();
        record::offers(&offered, 0x0403)
            .then(|| Box::new(P256Signer(self.scalar.clone())) as Box<dyn Signer>)
    }
    fn algorithm(&self) -> SignatureAlgorithm {
        SignatureAlgorithm::ECDSA
    }
    fn public_key(&self) -> Option<pki_types::SubjectPublicKeyInfoDer<'_>> {
        Some(self.spki.clone())
    }
}
impl Signer for P256Signer {
    fn scheme(&self) -> SignatureScheme {
        SignatureScheme::ECDSA_NISTP256_SHA256
    }
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        let invalid = |_| Error::General("P-256 signing failed".into());
        let private =
            libcrux_ecdsa::p256::PrivateKey::try_from(self.0.as_slice()).map_err(invalid)?;
        let signature = libcrux_ecdsa::p256::rand::sign(
            libcrux_sha2::Algorithm::Sha256,
            message,
            &private,
            &mut OsRandom,
        )
        .map_err(invalid)?;
        Ok(signature.to_der().to_vec())
    }
}

pub fn load_config(directory: &Path) -> io::Result<Arc<rustls::ServerConfig>> {
    let certs = pki_types::CertificateDer::pem_file_iter(directory.join("cert.pem"))
        .map_err(io::Error::other)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(io::Error::other)?;
    let key = PrivateKeyDer::from_pem_file(directory.join("key.pem")).map_err(io::Error::other)?;
    server_config(certs, key).map_err(io::Error::other)
}

pub fn server_config(
    certs: Vec<pki_types::CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) -> Result<Arc<rustls::ServerConfig>, Error> {
    let mut config = rustls::ServerConfig::builder_with_provider(Arc::new(provider()))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .and_then(|b| b.with_no_client_auth().with_single_cert(certs, key))?;
    config.alpn_protocols = vec![b"http/1.1".to_vec()];
    // Disable resumption: no ticket-encryption or cache operation needs another provider.
    config.session_storage = Arc::new(rustls::server::NoServerSessionStorage {});
    config.send_tls13_tickets = 0;
    Ok(Arc::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::KeyProvider;
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        sync::atomic::Ordering,
        time::Duration,
    };

    struct CertificateKey {
        key: Arc<dyn SigningKey>,
        public: Vec<u8>,
    }
    impl rcgen::PublicKeyData for CertificateKey {
        fn der_bytes(&self) -> &[u8] {
            &self.public
        }
        fn algorithm(&self) -> &'static rcgen::SignatureAlgorithm {
            &rcgen::PKCS_ECDSA_P256_SHA256
        }
    }
    impl rcgen::SigningKey for CertificateKey {
        fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, rcgen::Error> {
            Ok(self
                .key
                .choose_scheme(&[SignatureScheme::ECDSA_NISTP256_SHA256])
                .unwrap()
                .sign(msg)
                .unwrap())
        }
    }
    fn configs() -> (Arc<rustls::ServerConfig>, Arc<rustls::ClientConfig>) {
        use pkcs8::der::{asn1::AnyRef, Encode};
        let private = libcrux_ecdsa::p256::PrivateKey::random(&mut OsRandom).unwrap();
        let mut public = vec![4];
        public.extend_from_slice(&private.public_key().unwrap().0);
        let curve = pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.3.1.7");
        let ec = sec1::EcPrivateKey {
            private_key: private.as_ref(),
            parameters: None,
            public_key: None,
        };
        let ec_der = Zeroizing::new(ec.to_der().unwrap());
        let curve_der = curve.to_der().unwrap();
        let alg = pkcs8::AlgorithmIdentifierRef {
            oid: pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.2.1"),
            parameters: Some(AnyRef::try_from(curve_der.as_slice()).unwrap()),
        };
        let info = pkcs8::PrivateKeyInfo::new(alg, &ec_der);
        let der = pki_types::PrivatePkcs8KeyDer::from(info.to_der().unwrap());
        let key = P256KeyProvider
            .load_private_key(PrivateKeyDer::Pkcs8(der.clone_key()))
            .unwrap();
        let certificate_key = CertificateKey { key, public };
        let mut params = rcgen::CertificateParams::new(vec!["localhost".into()]).unwrap();
        params.serial_number = Some(1u64.into());
        let cert = params.self_signed(&certificate_key).unwrap().der().clone();
        let server = server_config(vec![cert.clone()], PrivateKeyDer::Pkcs8(der)).unwrap();
        let mut roots = rustls::RootCertStore::empty();
        roots.add(cert).unwrap();
        let mut client = rustls::ClientConfig::builder_with_provider(Arc::new(provider()))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .unwrap()
            .with_root_certificates(roots)
            .with_no_client_auth();
        client.alpn_protocols = vec![b"http/1.1".to_vec()];
        (server, Arc::new(client))
    }
    fn exchange(
        server: Arc<rustls::ServerConfig>,
        client: Arc<rustls::ClientConfig>,
        request: &[u8],
        chunk: usize,
    ) -> (Vec<u8>, usize) {
        exchange_policy(
            server,
            client,
            request,
            chunk,
            transport_core::http::Policy {
                unknown_drop: false,
                guarded_drop: 0,
                log_limit: 60,
            },
        )
    }
    fn exchange_policy(
        server: Arc<rustls::ServerConfig>,
        client: Arc<rustls::ClientConfig>,
        request: &[u8],
        chunk: usize,
        policy: transport_core::http::Policy,
    ) -> (Vec<u8>, usize) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let worker = std::thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            let state = crate::ServerState::with_policy(policy);
            crate::transport::serve_tcp(socket, Some(server), &state, std::time::Instant::now())
                .unwrap();
            state.handler_calls.load(Ordering::Relaxed)
        });
        let socket = TcpStream::connect(addr).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(4)))
            .unwrap();
        socket
            .set_write_timeout(Some(Duration::from_secs(4)))
            .unwrap();
        let conn = rustls::ClientConnection::new(client, "localhost".try_into().unwrap()).unwrap();
        let mut stream = rustls::StreamOwned::new(conn, socket);
        stream.conn.complete_io(&mut stream.sock).unwrap();
        assert_eq!(
            stream.conn.protocol_version(),
            Some(ProtocolVersion::TLSv1_3)
        );
        assert_eq!(
            stream.conn.negotiated_cipher_suite().unwrap().suite(),
            CipherSuite::TLS13_CHACHA20_POLY1305_SHA256
        );
        assert_eq!(stream.conn.alpn_protocol(), Some(b"http/1.1".as_slice()));
        assert_eq!(
            stream.conn.negotiated_key_exchange_group().unwrap().name(),
            NamedGroup::X25519
        );
        let parts: Vec<_> = request.chunks(chunk).collect();
        if parts.is_empty() {
            stream.conn.send_close_notify();
            stream.flush().unwrap();
        }
        for (index, part) in parts.iter().enumerate() {
            stream.conn.writer().write_all(part).unwrap();
            if index + 1 == parts.len() {
                stream.conn.send_close_notify();
            }
            stream.flush().unwrap();
        }
        let mut reply = Vec::new();
        stream.read_to_end(&mut reply).unwrap();
        (reply, worker.join().unwrap())
    }
    #[test]
    fn idle_and_partial_deadlines_tcp_and_tls() {
        // Real production durations on real sockets, in parallel to bound suite time.
        let mut cases = Vec::new();
        for tls in [false, true] {
            for scenario in ["delayed", "idle", "partial", "handshake_budget"] {
                if !tls && scenario == "handshake_budget" {
                    continue;
                }
                cases.push(std::thread::spawn(move || {
                    let (server, client) = configs();
                    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
                    let address = listener.local_addr().unwrap();
                    let worker = std::thread::spawn(move || {
                        let (socket, _) = listener.accept().unwrap();
                        let state = crate::ServerState::with_policy(transport_core::http::Policy {
                            unknown_drop: false,
                            guarded_drop: 0,
                            log_limit: 60,
                        });
                        crate::transport::serve_tcp(
                            socket,
                            tls.then_some(server),
                            &state,
                            std::time::Instant::now(),
                        )
                        .unwrap();
                        state.handler_calls.load(Ordering::Relaxed)
                    });
                    let socket = TcpStream::connect(address).unwrap();
                    socket
                        .set_read_timeout(Some(Duration::from_secs(20)))
                        .unwrap();
                    let start = std::time::Instant::now();
                    if scenario == "handshake_budget" {
                        std::thread::sleep(Duration::from_secs(3));
                    }
                    let mut stream: Box<dyn TestStream> = if tls {
                        let conn =
                            rustls::ClientConnection::new(client, "localhost".try_into().unwrap())
                                .unwrap();
                        let mut stream = rustls::StreamOwned::new(conn, socket);
                        stream.conn.complete_io(&mut stream.sock).unwrap();
                        Box::new(stream)
                    } else {
                        Box::new(socket)
                    };
                    match scenario {
                        "delayed" => {
                            std::thread::sleep(Duration::from_secs(3));
                            stream
                                .write_all(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n")
                                .unwrap();
                            stream.flush().unwrap();
                        }
                        "partial" => {
                            stream.write_all(b"GET / HTTP/1.1\r\nHost:").unwrap();
                            stream.flush().unwrap();
                            std::thread::sleep(Duration::from_secs(3));
                        }
                        "idle" => std::thread::sleep(Duration::from_secs(16)),
                        _ => {}
                    }
                    let mut response = Vec::new();
                    stream.read_to_end(&mut response).unwrap();
                    let calls = worker.join().unwrap();
                    match scenario {
                        "delayed" => {
                            assert!(response.starts_with(b"HTTP/1.1 200 OK\r\n"));
                            assert_eq!(calls, 1);
                        }
                        "partial" => {
                            assert!(response.starts_with(b"HTTP/1.1 400 Bad Request\r\n"));
                            assert_eq!(calls, 0);
                        }
                        _ => {
                            assert!(response.is_empty());
                            assert_eq!(calls, 0);
                        }
                    }
                    // Includes client-side sleep; worker must finish near absolute15s,
                    // even when the TLS handshake consumed the first3s.
                    if scenario == "handshake_budget" {
                        assert!(start.elapsed() >= Duration::from_secs(14));
                        assert!(start.elapsed() < Duration::from_secs(17));
                    }
                }));
            }
        }
        for case in cases {
            case.join().unwrap();
        }
    }

    include!("io_deadline_tests.rs");

    trait TestStream: Read + Write {}
    impl<T: Read + Write> TestStream for T {}

    #[test]
    fn idle_handshake_budget_is_absolute() {
        for trickle in [false, true] {
            let (server, _) = configs();
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let worker = std::thread::spawn(move || {
                let (socket, _) = listener.accept().unwrap();
                let start = std::time::Instant::now();
                crate::transport::serve_tcp(
                    socket,
                    Some(server),
                    &crate::ServerState::with_policy(transport_core::http::Policy {
                        unknown_drop: false,
                        guarded_drop: 0,
                        log_limit: 60,
                    }),
                    start,
                )
                .unwrap();
                start.elapsed()
            });
            let mut socket = TcpStream::connect(address).unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(18)))
                .unwrap();
            if trickle {
                // Incomplete TLS record, sent slowly: raw bytes must not reset idle.
                for byte in [22, 3, 3, 0] {
                    socket.write_all(&[byte]).unwrap();
                    std::thread::sleep(Duration::from_secs(3));
                }
            }
            let mut response = Vec::new();
            socket.read_to_end(&mut response).unwrap();
            assert!(response.is_empty());
            let elapsed = worker.join().unwrap();
            assert!(elapsed >= Duration::from_secs(14));
            assert!(elapsed < Duration::from_secs(17), "{elapsed:?}");
        }
    }

    #[test]
    fn tls_routes_and_safari() {
        let (server, client) = configs();
        for (wire, status) in [
            (
                b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n".as_slice(),
                "200 OK",
            ),
            (
                include_bytes!("../tests/safari-root.http").as_slice(),
                "200 OK",
            ),
            (
                include_bytes!("../tests/safari-favicon.http").as_slice(),
                "404 Not Found",
            ),
            (
                include_bytes!("../tests/safari-https-root.http").as_slice(),
                "200 OK",
            ),
            (
                include_bytes!("../tests/safari-https-favicon.http").as_slice(),
                "404 Not Found",
            ),
            (
                include_bytes!("../tests/firefox-https-root.http").as_slice(),
                "200 OK",
            ),
            (
                include_bytes!("../tests/firefox-https-favicon.http").as_slice(),
                "404 Not Found",
            ),
            (
                include_bytes!("../tests/brave-https-root.http").as_slice(),
                "200 OK",
            ),
            (
                include_bytes!("../tests/brave-https-favicon.http").as_slice(),
                "404 Not Found",
            ),
            (
                include_bytes!("../tests/brave-cache-disabled-https-root.http").as_slice(),
                "200 OK",
            ),
        ] {
            for chunk in [usize::MAX, 1, 7, 1024] {
                let (reply, calls) = exchange(server.clone(), client.clone(), wire, chunk);
                assert!(reply.starts_with(format!("HTTP/1.1 {status}\r\n").as_bytes()));
                assert_eq!(calls, 1);
            }
        }
    }
    #[test]
    fn default_policy_browser_fixtures_and_sensitive_names() {
        let (server, client) = configs();
        let default = transport_core::http::Policy::builtin();
        for wire in [
            include_bytes!("../tests/safari-https-root.http").as_slice(),
            include_bytes!("../tests/firefox-https-root.http").as_slice(),
            include_bytes!("../tests/brave-https-root.http").as_slice(),
            include_bytes!("../tests/brave-cache-disabled-https-root.http").as_slice(),
            include_bytes!("../tests/brave-dnt-https-root.http").as_slice(),
        ] {
            for chunk in [usize::MAX, 1, 7, 1024] {
                let (reply, calls) =
                    exchange_policy(server.clone(), client.clone(), wire, chunk, default);
                assert!(reply.starts_with(b"HTTP/1.1 200"));
                assert_eq!(calls, 1);
            }
        }
        for name in [
            "Content-Length",
            "Transfer-Encoding",
            "TE",
            "Trailer",
            "Expect",
            "Upgrade",
            "HTTP2-Settings",
            "Proxy-Connection",
            "Keep-Alive",
            "Cookie",
            "Authorization",
            "Proxy-Authorization",
            "Range",
            "If-Range",
            "If-Match",
            "If-None-Match",
            "If-Modified-Since",
            "If-Unmodified-Since",
            "Forwarded",
            "X-Forwarded-For",
            "X-Forwarded-Host",
            "X-Forwarded-Proto",
            "X-Real-IP",
            "Via",
            "Max-Forwards",
        ] {
            let name: String = name
                .bytes()
                .enumerate()
                .map(|(i, b)| {
                    char::from(if i % 2 == 0 {
                        b.to_ascii_lowercase()
                    } else {
                        b.to_ascii_uppercase()
                    })
                })
                .collect();
            let wire = format!("GET / HTTP/1.1\r\n{name}: synthetic\r\n\r\n");
            for chunk in [usize::MAX, 1, 7, 1024] {
                let (reply, calls) = exchange_policy(
                    server.clone(),
                    client.clone(),
                    wire.as_bytes(),
                    chunk,
                    default,
                );
                assert!(reply.starts_with(b"HTTP/1.1 400"));
                assert_eq!(calls, 0);
            }
        }
        let cookie = transport_core::http::parse_config(b"guarded Cookie drop").unwrap();
        let (reply, calls) = exchange_policy(
            server.clone(),
            client.clone(),
            b"GET / HTTP/1.1\r\nCookie: synthetic\r\n\r\n",
            1,
            cookie,
        );
        assert!(reply.starts_with(b"HTTP/1.1 200"));
        assert_eq!(calls, 1);
        let (reply, calls) = exchange(
            server,
            client,
            include_bytes!("../tests/brave-dnt-https-root.http"),
            7,
        );
        assert!(reply.starts_with(b"HTTP/1.1 400"));
        assert_eq!(calls, 0);
    }
    #[test]
    fn tls_case_and_count_boundaries() {
        let (server, client) = configs();
        for name in [
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
        ] {
            for form in [
                name.to_ascii_lowercase(),
                name.to_ascii_uppercase(),
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
                    .collect(),
            ] {
                let wire = format!("GET / HTTP/1.1\r\n{form}: x\r\n\r\n");
                for chunk in [usize::MAX, 1, 7, 1024] {
                    let (reply, calls) =
                        exchange(server.clone(), client.clone(), wire.as_bytes(), chunk);
                    assert!(reply.starts_with(b"HTTP/1.1 200"));
                    assert_eq!(calls, 1);
                }
            }
        }
        for name in ["Pragmas", "Pragm"] {
            for chunk in [usize::MAX, 1, 7, 1024] {
                let wire = format!("GET / HTTP/1.1\r\n{name}: x\r\n\r\n");
                let (reply, calls) =
                    exchange(server.clone(), client.clone(), wire.as_bytes(), chunk);
                assert!(reply.starts_with(b"HTTP/1.1 400"));
                assert_eq!(calls, 0);
            }
        }
        for count in [24, 25] {
            let wire = format!("GET / HTTP/1.1\r\n{}\r\n", "aCcEpT: x\r\n".repeat(count));
            for chunk in [usize::MAX, 1, 7, 1024] {
                let (reply, calls) =
                    exchange(server.clone(), client.clone(), wire.as_bytes(), chunk);
                assert!(reply.starts_with(if count == 24 {
                    b"HTTP/1.1 200"
                } else {
                    b"HTTP/1.1 400"
                }));
                assert_eq!(calls, if count == 24 { 1 } else { 0 });
            }
        }
    }
    #[test]
    fn malformed_tls_never_dispatches() {
        let (server, client) = configs();
        let mut corpus: Vec<Vec<u8>> = include_str!("../tests/malformed.hex")
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| {
                l.split_whitespace()
                    .map(|b| u8::from_str_radix(b, 16).unwrap())
                    .collect()
            })
            .collect();
        corpus.push(vec![]);
        assert_eq!(corpus.len(), 48);
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
            for line in [format!("{name}: "), format!("{name}: {}", "x".repeat(257))] {
                corpus.push(format!("GET / HTTP/1.1\r\n{line}\r\n\r\n").into_bytes());
            }
        }
        for name in [
            "Accept-Languages",
            "Refere",
            "Sec-Fetch-Destination",
            "Sec-Fetch-",
            "Sec-Fetch-Users",
            "Sec-Fetch-Use",
            "sec-ch-ua-mobil",
            "Pragmas",
            "Pragm",
            "content-length",
            "TRANSFER-ENCODING",
            "cOoKiE",
        ] {
            corpus.push(format!("GET / HTTP/1.1\r\n{name}: x\r\n\r\n").into_bytes());
        }
        for (i, request) in corpus.iter().enumerate() {
            // One TLS application write keeps coalesced trailing bytes in the same read.
            let (reply, calls) = exchange(server.clone(), client.clone(), request, usize::MAX);
            assert!(
                reply.starts_with(b"HTTP/1.1 400 Bad Request\r\n"),
                "case {i}"
            );
            assert_eq!(calls, 0, "case {i}");
        }
    }
    #[test]
    fn aead_rejects_tampered_record_and_x25519_rejects_low_order() {
        use crypto::SupportedKxGroup;
        let plain = b"authenticated";
        let key = [7; 32];
        let nonce = [3; 12];
        let aad = b"header";
        let mut ciphertext = vec![0; plain.len() + 16];
        libcrux_chacha20poly1305::encrypt(&key, plain, &mut ciphertext, aad, &nonce).unwrap();
        ciphertext[0] ^= 1;
        let mut out = vec![0; plain.len()];
        assert!(
            libcrux_chacha20poly1305::decrypt(&key, &mut out, &ciphertext, aad, &nonce).is_err()
        );
        assert!(X25519.start().unwrap().complete(&[0; 32]).is_err());
        assert!(X25519.start().unwrap().complete(&[1; 31]).is_err());
    }
    #[test]
    fn hkdf_rfc5869_case1_and_hash_fork() {
        use crypto::{
            hash::{Context, Hash},
            tls13::Hkdf as _,
        };
        let expander = Hkdf.extract_from_secret(Some(&(0..=12).collect::<Vec<u8>>()), &[0x0b; 22]);
        let mut out = [0; 42];
        expander
            .expand_slice(
                &[
                    &[0xf0, 0xf1, 0xf2],
                    &[0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9],
                ],
                &mut out,
            )
            .unwrap();
        let expected =
            "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865";
        assert_eq!(
            out.iter().map(|b| format!("{b:02x}")).collect::<String>(),
            expected
        );
        assert!(expander.expand_slice(&[], &mut vec![0; 8161]).is_err());
        let mut hash = HashContext(libcrux_sha2::Sha256::new());
        hash.update(b"ab");
        let mut fork = hash.fork();
        fork.update(b"c");
        assert_eq!(fork.finish().as_ref(), Sha256.hash(b"abc").as_ref());
        assert_eq!(hash.fork_finish().as_ref(), Sha256.hash(b"ab").as_ref());
    }
}

#[cfg(test)]
#[path = "record_tests.rs"]
mod record_tests;
