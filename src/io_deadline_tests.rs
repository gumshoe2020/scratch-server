// Included in tls::tests to reuse the existing in-memory certificate/config.
#[test]
fn stage16_request_deadlines_plain_and_tls() {
    let cases: Vec<_> = [false, true].into_iter().map(|tls| std::thread::spawn(move || {
        for scenario in ["trickle", "eight_seconds", "idle_three", "idle_seventeen"] {
            let (server, client) = configs();
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let worker = std::thread::spawn(move || {
                let (socket, _) = listener.accept().unwrap();
                let state = crate::ServerState::default();
                crate::transport::serve_tcp(socket, tls.then_some(server), &state,
                    std::time::Instant::now()).unwrap();
                state.handler_calls.load(Ordering::Relaxed)
            });
            let socket = TcpStream::connect(address).unwrap();
            socket.set_read_timeout(Some(Duration::from_millis(50))).unwrap();
            socket.set_write_timeout(Some(Duration::from_secs(2))).unwrap();
            let mut stream: Box<dyn TestStream> = if tls {
                // Complete the handshake before applying the polling timeout.
                socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
                let conn = rustls::ClientConnection::new(client, "localhost".try_into().unwrap()).unwrap();
                let mut stream = rustls::StreamOwned::new(conn, socket);
                stream.conn.complete_io(&mut stream.sock).unwrap();
                stream.sock.set_read_timeout(Some(Duration::from_millis(50))).unwrap();
                Box::new(stream)
            } else { Box::new(socket) };
            let wire = b"GET / HTTP/1.1\r\nHost: x\r\n\r\n";
            let start = std::time::Instant::now();
            let mut next = 0;
            let mut response = Vec::new();
            let mut first = None;
            loop {
                let elapsed = start.elapsed();
                let send = match scenario {
                    "trickle" => next < wire.len() && elapsed >= Duration::from_millis(1500 * next as u64),
                    "eight_seconds" => next < 9 && elapsed >= Duration::from_secs(next as u64),
                    "idle_three" => next == 0 && elapsed >= Duration::from_secs(3),
                    _ => false,
                };
                if send {
                    first.get_or_insert_with(std::time::Instant::now);
                    match scenario {
                        "trickle" => stream.write_all(&wire[next..next+1]).unwrap(),
                        "eight_seconds" => {
                            let a = wire.len() * next / 9;
                            let b = wire.len() * (next+1) / 9;
                            stream.write_all(&wire[a..b]).unwrap();
                        }
                        _ => stream.write_all(wire).unwrap(),
                    }
                    stream.flush().unwrap();
                    next += 1;
                }
                // Observe the actual response while sending, rather than waiting
                // until the next scheduled byte to discover that it timed out.
                let mut buf = [0u8; 1024];
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => response.extend_from_slice(&buf[..n]),
                    Err(e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {},
                    Err(e) => panic!("deadline read error: {:?}", e.kind()),
                }
                assert!(start.elapsed() < Duration::from_secs(20));
            }
            let elapsed = first.map(|t| t.elapsed()).unwrap_or_else(|| start.elapsed());
            let calls = worker.join().unwrap();
            match scenario {
                "trickle" => {
                    assert!(response.starts_with(b"HTTP/1.1 400 Bad Request\r\n"));
                    assert!(elapsed >= Duration::from_secs(10) && elapsed < Duration::from_secs(12), "{elapsed:?}");
                    assert_eq!(calls, 0);
                }
                "idle_seventeen" => {
                    // Observe closure at the idle deadline, then remain idle for
                    // the full required seventeen seconds before asserting.
                    std::thread::sleep(Duration::from_secs(17).saturating_sub(start.elapsed()));
                    assert!(response.is_empty());
                    assert_eq!(calls, 0);
                }
                _ => {
                    assert!(response.starts_with(b"HTTP/1.1 200 OK\r\n"));
                    assert_eq!(calls, 1);
                    if scenario == "eight_seconds" { assert!(elapsed >= Duration::from_secs(8)); }
                }
            }
            eprintln!("stage16 deadline tls={tls} scenario={scenario} response_seconds={:.3}", elapsed.as_secs_f64());
        }
    })).collect();
    for case in cases { case.join().unwrap(); }
}
