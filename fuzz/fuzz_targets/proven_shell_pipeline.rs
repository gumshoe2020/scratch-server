//! Arbitrary bytes through the production read path: rustls::StreamOwned over an in-memory
//! socket into scratch_server::serve_connection (TLS shell plus proven machine), followed by
//! the same close_notify tail as transport::serve_tcp. serve_tcp itself needs a TcpStream
//! for its deadlines, so the socket deadline wrapper is the one part not exercised here.
//! Byte 0: low bit 0 = raw mode, the rest of the input is the client's wire bytes;
//!         low bit 1 = client mode, a real in-process rustls client (our provider) completes
//!         the handshake and then sends the rest of the input as HTTP plaintext.
//!         Bit 1 asks the client for close_notify before EOF; bits 2..8 seed chunking.
#![no_main]
use libfuzzer_sys::fuzz_target;
use rustls::{ClientConfig, ClientConnection, ServerConnection, StreamOwned};
use scratch_server::{serve_connection, ServerState};
use scratch_server_fuzz::{client, client_config, server_config, Chunks};
use std::io::{self, Read, Write};
use std::sync::{Arc, OnceLock};

fn config() -> &'static Arc<ClientConfig> {
    static CLIENT: OnceLock<Arc<ClientConfig>> = OnceLock::new();
    CLIENT.get_or_init(|| client_config(false))
}

/// The client's bytes, arriving in fuzzer-chosen chunks, then EOF. Server output is dropped.
struct RawSocket<'a> {
    wire: &'a [u8],
    chunks: Chunks,
}
impl Read for RawSocket<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.chunks.next(self.wire.len()).min(buf.len());
        buf[..n].copy_from_slice(&self.wire[..n]);
        self.wire = &self.wire[n..];
        Ok(n)
    }
}
impl Write for RawSocket<'_> {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A socket whose far end is a rustls client: server writes go into the client, and server
/// reads return whatever the client sends next (handshake, then the plaintext in chunks).
struct PeerSocket<'a> {
    peer: ClientConnection,
    plain: &'a [u8],
    chunks: Chunks,
    close_notify: bool,
    broken: bool,
}
impl Read for PeerSocket<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.peer.wants_write() {
                return self.peer.write_tls(&mut &mut buf[..]);
            }
            if self.broken || self.peer.is_handshaking() {
                return Ok(0);
            }
            if !self.plain.is_empty() {
                let n = self.chunks.next(self.plain.len());
                let sent = self.peer.writer().write(&self.plain[..n])?;
                if sent == 0 {
                    return Ok(0);
                }
                self.plain = &self.plain[sent..];
                continue;
            }
            if self.close_notify {
                self.close_notify = false;
                self.peer.send_close_notify();
                continue;
            }
            return Ok(0);
        }
    }
}
impl Write for PeerSocket<'_> {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        let mut rd = b;
        while !rd.is_empty() && !self.broken {
            match self.peer.read_tls(&mut rd) {
                Ok(0) | Err(_) => self.broken = true,
                Ok(_) => self.broken = self.peer.process_new_packets().is_err(),
            }
            let mut sink = [0u8; 4096];
            while let Ok(n) = self.peer.reader().read(&mut sink) {
                if n == 0 {
                    break;
                }
            }
        }
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// The TLS branch of transport::serve_tcp, minus the TcpStream deadline wrapper.
fn serve(socket: impl Read + Write, state: &ServerState) {
    let conn = ServerConnection::new(server_config()).unwrap();
    let mut stream = StreamOwned::new(conn, socket);
    if serve_connection(&mut stream, state).is_err() || stream.conn.is_handshaking() {
        return;
    }
    stream.conn.send_close_notify();
    let _ = stream.flush();
}

fuzz_target!(|data: &[u8]| {
    static STATE: OnceLock<ServerState> = OnceLock::new();
    let state = STATE.get_or_init(ServerState::default);
    let Some((&mode, rest)) = data.split_first() else {
        return;
    };
    let chunks = Chunks::new(mode >> 2);
    if mode & 1 == 0 {
        serve(RawSocket { wire: rest, chunks }, state);
    } else {
        let socket = PeerSocket {
            peer: client(config()),
            plain: rest,
            chunks,
            close_notify: mode & 2 != 0,
            broken: false,
        };
        serve(socket, state);
    }
});
