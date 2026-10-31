//! Shell-free server, one bounded request per connection.
use rustls::pki_types::pem::PemObject;
use scratch_server::ServerState;
use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::ExitCode,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

fn port() -> std::io::Result<u16> {
    let value = env::var("SCRATCH_PORT").ok();
    transport_core::io::parse_port(value.as_deref().map(str::as_bytes))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid SCRATCH_PORT"))
}

fn main() -> ExitCode {
    if env::args().any(|a| a == "--healthcheck") {
        return healthcheck();
    }
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("server: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> std::io::Result<()> {
    let (policy, origin) = match env::var_os("SCRATCH_HEADER_POLICY") {
        None => (transport_core::http::Policy::builtin(), "builtin"),
        Some(path) => {
            let mut bytes = Vec::new();
            std::fs::File::open(path)
                .map_err(|_| std::io::Error::other("header policy load failed"))?
                .take(4097)
                .read_to_end(&mut bytes)?;
            let policy = transport_core::http::parse_config(&bytes)
                .ok_or_else(|| std::io::Error::other("invalid header policy"))?;
            (policy, "file")
        }
    };
    let guarded = [
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
    ];
    let dropped: Vec<_> = guarded
        .iter()
        .enumerate()
        .filter_map(|(i, name)| (policy.guarded_drop & (1 << i) != 0).then_some(*name))
        .collect();
    println!(
        "policy={origin} unknown={} guarded-drop={} log-unknown-per-minute={}",
        if policy.unknown_drop {
            "drop"
        } else {
            "reject"
        },
        dropped.join(","),
        policy.log_limit
    );
    let directory = env::var("SCRATCH_TLS_DIR").unwrap_or_else(|_| "/tls".into());
    let config = scratch_server::tls::load_config(std::path::Path::new(&directory))?;
    let listener = TcpListener::bind(("0.0.0.0", port()?))?;
    let state = Arc::new(ServerState::with_policy(policy));
    let active = Arc::new(AtomicUsize::new(0));
    println!("listening on {}", listener.local_addr()?);
    for connection in listener.incoming() {
        let stream = connection?;
        let accepted = Instant::now();
        // Bound concurrent threads; excess connections are closed without dispatch.
        if active
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, transport_core::io::admit)
            .is_err()
        {
            println!("closed reason=capacity");
            continue;
        }
        let state = Arc::clone(&state);
        let active = Arc::clone(&active);
        let config = Arc::clone(&config);
        std::thread::spawn(move || {
            let result =
                scratch_server::transport::serve_tcp(stream, Some(config), &state, accepted);
            if let Err(e) = result {
                eprintln!("closed reason=transport kind={:?}", e.kind());
            }
            let _ = active.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                Some(transport_core::io::release(n))
            });
        });
    }
    Ok(())
}

/// Authenticate the local TLS endpoint using its mounted certificate.
fn healthcheck() -> ExitCode {
    let check = || -> std::io::Result<bool> {
        let stream = TcpStream::connect_timeout(
            &(([127, 0, 0, 1], port()?).into()),
            Duration::from_secs(2),
        )?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;
        let directory = env::var("SCRATCH_TLS_DIR").unwrap_or_else(|_| "/tls".into());
        let mut roots = rustls::RootCertStore::empty();
        for cert in rustls::pki_types::CertificateDer::pem_file_iter(
            std::path::Path::new(&directory).join("cert.pem"),
        )
        .map_err(std::io::Error::other)?
        {
            roots
                .add(cert.map_err(std::io::Error::other)?)
                .map_err(std::io::Error::other)?;
        }
        let mut config =
            rustls::ClientConfig::builder_with_provider(Arc::new(scratch_server::tls::provider()))
                .with_protocol_versions(&[&rustls::version::TLS13])
                .map_err(std::io::Error::other)?
                .with_root_certificates(roots)
                .with_no_client_auth();
        config.alpn_protocols = vec![b"http/1.1".to_vec()];
        let conn = rustls::ClientConnection::new(Arc::new(config), "localhost".try_into().unwrap())
            .map_err(std::io::Error::other)?;
        let mut stream = rustls::StreamOwned::new(conn, stream);
        stream.write_all(b"GET /health HTTP/1.0\r\nHost: localhost\r\n\r\n")?;
        let mut reply = String::new();
        stream.read_to_string(&mut reply)?;
        Ok(reply.starts_with("HTTP/1.1 200 OK\r\n") && reply.ends_with("\r\n\r\nok"))
    };
    if check().unwrap_or(false) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod io_tests {
    #[test]
    fn stage16_port_unset_non_utf8_and_error() {
        use std::os::unix::ffi::OsStringExt;
        let previous = std::env::var_os("SCRATCH_PORT");
        std::env::remove_var("SCRATCH_PORT");
        assert_eq!(super::port().unwrap(), 8080);
        std::env::set_var("SCRATCH_PORT", std::ffi::OsString::from_vec(vec![255]));
        assert_eq!(super::port().unwrap(), 8080);
        std::env::set_var("SCRATCH_PORT", "+65535");
        assert_eq!(super::port().unwrap(), 65535);
        std::env::set_var("SCRATCH_PORT", "65536");
        let error = super::port().unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        assert_eq!(error.to_string(), "invalid SCRATCH_PORT");
        match previous {
            Some(value) => std::env::set_var("SCRATCH_PORT", value),
            None => std::env::remove_var("SCRATCH_PORT"),
        }
    }
}
