//! Trusted socket deadlines shared by plaintext and TLS request paths.
use crate::{serve_with_started, ServerState};
use std::{
    cell::Cell,
    io::{self, Read, Write},
    net::TcpStream,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

pub const IDLE_TIMEOUT: Duration = Duration::from_secs(15);
pub const READ_TIMEOUT: Duration = Duration::from_secs(2);

struct DeadlineSocket {
    socket: TcpStream,
    idle_until: Instant,
    started: Rc<Cell<bool>>,
}

impl Read for DeadlineSocket {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        let timeout = if self.started.get() {
            READ_TIMEOUT
        } else {
            self.idle_until
                .checked_duration_since(Instant::now())
                .filter(|d| !d.is_zero())
                .ok_or_else(|| io::Error::from(io::ErrorKind::TimedOut))?
        };
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
    let started = Rc::new(Cell::new(false));
    let socket = DeadlineSocket {
        socket,
        idle_until: accepted + IDLE_TIMEOUT,
        started: Rc::clone(&started),
    };
    if let Some(config) = tls {
        let conn = rustls::ServerConnection::new(config).map_err(io::Error::other)?;
        let mut stream = rustls::StreamOwned::new(conn, socket);
        serve_with_started(&mut stream, state, || started.set(true))?;
        if stream.conn.is_handshaking() {
            return Ok(());
        }
        stream.conn.send_close_notify();
        stream.flush()
    } else {
        serve_with_started(&mut { socket }, state, || started.set(true))
    }
}
