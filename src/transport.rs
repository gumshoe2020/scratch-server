//! Trusted socket deadlines shared by plaintext and TLS request paths.
use crate::{serve_with_started, ServerState};
use transport_core::io as core_io;
use std::{
    cell::Cell,
    io::{self, Read, Write},
    net::TcpStream,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

pub const IDLE_TIMEOUT: Duration = Duration::from_nanos(core_io::IDLE_NS);
pub const READ_TIMEOUT: Duration = Duration::from_nanos(core_io::READ_NS);

struct DeadlineSocket {
    socket: TcpStream,
    accepted: Instant,
    started: Rc<Cell<Option<u64>>>,
}

impl Read for DeadlineSocket {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        let elapsed = elapsed_ns(self.accepted);
        let timeout = core_io::read_timeout(elapsed, self.started.get())
            .map(Duration::from_nanos)
            .ok_or_else(|| io::Error::from(io::ErrorKind::TimedOut))?;
        self.socket.set_read_timeout(Some(timeout))?;
        self.socket.read(bytes)
    }
}

impl Write for DeadlineSocket {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.socket.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.socket.flush()
    }
}

/// The idle budget starts at accept, including handshake reads. Only decrypted
/// HTTP bytes switch to the short timeout; TLS records cannot extend idle life.
pub fn serve_tcp(
    socket: TcpStream,
    tls: Option<Arc<rustls::ServerConfig>>,
    state: &ServerState,
    accepted: Instant,
) -> io::Result<()> {
    socket.set_write_timeout(Some(READ_TIMEOUT))?;
    let started = Rc::new(Cell::new(None));
    let socket = DeadlineSocket {
        socket,
        accepted,
        started: Rc::clone(&started),
    };
    let mark_started = || {
        if started.get().is_none() {
            started.set(Some(elapsed_ns(accepted)));
        }
    };
    if let Some(config) = tls {
        let conn = rustls::ServerConnection::new(config).map_err(io::Error::other)?;
        let mut stream = rustls::StreamOwned::new(conn, socket);
        serve_with_started(&mut stream, state, mark_started)?;
        if stream.conn.is_handshaking() {
            return Ok(());
        }
        stream.conn.send_close_notify();
        stream.flush()
    } else {
        serve_with_started(&mut { socket }, state, mark_started)
    }
}

// Instant and Duration stay in the trusted shell; the core sees integer ns.
fn elapsed_ns(accepted: Instant) -> u64 {
    u64::try_from(Instant::now().saturating_duration_since(accepted).as_nanos())
        .unwrap_or(u64::MAX)
}
