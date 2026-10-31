//! Pure startup and I/O decisions; clocks, syscalls and atomics stay in the shell.
#[path = "spec/io.rs"]
pub mod spec;
pub use self::spec::*;
use vstd::prelude::*;

verus! {
pub const IDLE_NS: u64 = 15_000_000_000;
pub const READ_NS: u64 = 2_000_000_000;
pub const REQUEST_NS: u64 = 10_000_000_000;
pub const MAX_CONNECTIONS: usize = 64;
pub const HASH_LEN: usize = 32;
pub const X25519_LEN: usize = 32;
pub const P256_POINT_LEN: usize = 65;

pub fn read_timeout(elapsed: u64, first_byte: Option<u64>) -> (out: Option<u64>)
    ensures out == timeout_spec(elapsed, first_byte),
        match out {
            Some(d) => d > 0 && match first_byte {
                None => elapsed as int + d <= IDLE_NS,
                Some(t) => d <= READ_NS && elapsed as int + d <= request_end(t),
            },
            None => match first_byte {
                None => elapsed >= IDLE_NS,
                Some(t) => elapsed >= request_end(t),
            },
        },
{
    match first_byte {
        None => if elapsed < IDLE_NS { Some(IDLE_NS - elapsed) } else { None },
        Some(t) => {
            let deadline = if t > u64::MAX - REQUEST_NS { u64::MAX } else { t + REQUEST_NS };
            if elapsed >= deadline { return None; }
            let left = deadline - elapsed;
            Some(if left < READ_NS { left } else { READ_NS })
        },
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReadResult { Data(usize), Interrupted, Timeout, Other }
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Step { Retry, Eof, Bytes(usize), ReadTimeout, Fail }
pub fn classify(r: ReadResult) -> (out: (Step, bool))
    ensures out.0 == match r {
        ReadResult::Data(n) => if n == 0 { Step::Eof } else { Step::Bytes(n) },
        ReadResult::Interrupted => Step::Retry,
        ReadResult::Timeout => Step::ReadTimeout,
        ReadResult::Other => Step::Fail,
    }, out.1 == (out.0 is Bytes),
{
    match r {
        ReadResult::Data(0) => (Step::Eof, false),
        ReadResult::Data(n) => (Step::Bytes(n), true),
        ReadResult::Interrupted => (Step::Retry, false),
        ReadResult::Timeout => (Step::ReadTimeout, false),
        ReadResult::Other => (Step::Fail, false),
    }
}

pub fn admit(active: usize) -> (out: Option<usize>)
    ensures out == admission_result(active),
        out is Some ==> 1 <= out->Some_0 <= MAX_CONNECTIONS,
{
    if active < MAX_CONNECTIONS { Some(active + 1) } else { None }
}
pub fn release(active: usize) -> (out: usize)
    ensures out == release_result(active), out <= active,
{
    if active == 0 { 0 } else { active - 1 }
}

pub fn parse_port(s: Option<&[u8]>) -> (out: Option<u16>)
    ensures match s {
        None => out == Some(8080),
        Some(bytes) => match out {
            Some(p) => port_form(bytes@, p as int),
            None => forall|p: u16| !port_form(bytes@, p as int),
        },
    },
{
    let bytes = match s { None => return Some(8080), Some(b) => b };
    let start = if bytes.len() > 0 && bytes[0] == b'+' { 1usize } else { 0usize };
    if start == bytes.len() { return None; }
    let mut i = start;
    let mut value = 0u32;
    while i < bytes.len()
        invariant s == Some(bytes), start == port_start(bytes@), start < bytes.len(),
            start <= i <= bytes.len(), value <= 65_535,
            digit_range(bytes@, start as int, i as int),
            value == crate::decimal_value(bytes@.subrange(start as int, i as int)),
        decreases bytes.len() - i,
    {
        let b = bytes[i];
        if b < b'0' || b > b'9' {
            assert forall|p: u16| !port_form(bytes@, p as int) by {
                if port_form(bytes@, p as int) {
                    assert(digit_range(bytes@, start as int, bytes.len() as int));
                    assert(48 <= bytes@[i as int] <= 57);
                }
            }
            return None;
        }
        let next = value * 10 + (b - b'0') as u32;
        proof {
            assert(bytes@.subrange(start as int, i+1).subrange(0, i-start) =~=
                bytes@.subrange(start as int, i as int));
            assert(bytes@.subrange(start as int, i+1).last() == b);
            assert(next == crate::decimal_value(bytes@.subrange(start as int, i+1)));
        }
        if next > 65_535 {
            proof {
                if digit_range(bytes@, start as int, bytes.len() as int) {
                    decimal_prefix(bytes@, start as int, i+1);
                }
                assert forall|p: u16| !port_form(bytes@, p as int) by {
                    if port_form(bytes@, p as int) {
                        assert(digit_range(bytes@, start as int, bytes.len() as int));
                        assert(crate::decimal_value(bytes@.subrange(start as int, bytes.len() as int)) >= next);
                    }
                }
            }
            return None;
        }
        value = next;
        i += 1;
    }
    Some(value as u16)
}

}
