use scratch_server::{serve_connection, ServerState};
use std::{
    io::{self, Cursor, Read, Write},
    sync::atomic::Ordering,
};
use transport_core::{
    app::{self, Route, Task},
    http::Request,
    Action, Reason,
};

struct Wire {
    input: Cursor<Vec<u8>>,
    output: Vec<u8>,
}
impl Read for Wire {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.input.read(bytes)
    }
}
impl Write for Wire {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.output.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
#[test]
fn shell_routes_get_and_head_with_exactly_one_handler_call() {
    for (path, status, body) in [
        ("/health", 200, b"ok".as_slice()),
        ("/absent", 404, b"not found\n"),
        ("/", 200, b"Hello from a FROM-scratch container.\n"),
    ] {
        let mut responses = Vec::new();
        for method in ["GET", "HEAD"] {
            let state = ServerState::default();
            let mut wire = Wire {
                input: Cursor::new(format!("{method} {path} HTTP/1.1\r\n\r\n").into_bytes()),
                output: Vec::new(),
            };
            serve_connection(&mut wire, &state).unwrap();
            assert!(state.handler_calls.load(Ordering::Relaxed) == 1);
            assert!(wire
                .output
                .starts_with(format!("HTTP/1.1 {status} ").as_bytes()));
            let boundary = wire
                .output
                .windows(4)
                .position(|w| w == b"\r\n\r\n")
                .unwrap()
                + 4;
            if method == "HEAD" {
                assert!(wire.output.len() == boundary);
            } else {
                assert!(wire.output[boundary..].starts_with(body));
            }
            responses.push((wire.output, boundary));
        }
        assert!(responses[0].0[..responses[0].1] == responses[1].0[..responses[1].1]);
    }
}
#[test]
fn plans_never_respond_or_increment_except_as_specified() {
    for action in [Action::ReadMore, Action::CloseSilently, Action::Done] {
        let p = app::plan(action);
        assert!(p.handler_increment == 0);
        assert!(app::task_response(&p.task, Route::Root, b"ignored").is_none());
    }
    for reason in [Reason::Grammar, Reason::Size, Reason::Slow] {
        let p = app::plan(Action::Reject(reason));
        assert!(p.handler_increment == 0);
        let bytes = app::task_response(&p.task, Route::Root, b"ignored").unwrap();
        assert!(bytes == b"HTTP/1.1 400 Bad Request\r\nContent-Length: 12\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nbad request\n");
    }
    for (path, r, expected) in [
        (b"/health".as_slice(), Route::Health, 200),
        (b"/", Route::Root, 200),
        (b"/health/", Route::NotFound, 404),
        (b"/HEALTH", Route::NotFound, 404),
        (b"", Route::NotFound, 404),
    ] {
        assert!(app::route(path) == r);
        let p = app::plan(Action::Dispatch(Request {
            head: false,
            path_start: 0,
            path_end: path.len(),
        }));
        assert!(p.handler_increment == 1 && matches!(p.task, Task::Dispatch(_)));
        let log = app::unknown_line(r, b"X\r\n =\x00Name");
        let response = app::task_response(&p.task, r, b"root body").unwrap();
        assert!(log.starts_with(format!("status={expected} ").as_bytes()));
        assert!(response.starts_with(format!("HTTP/1.1 {expected} ").as_bytes()));
        assert!(log.ends_with(b"name=XName"));
    }
}
