//! Pure startup and I/O decisions; clocks, syscalls and atomics stay in the shell.
use vstd::prelude::*;

verus! {
pub const IDLE_NS: u64 = 15_000_000_000;
pub const READ_NS: u64 = 2_000_000_000;
pub const REQUEST_NS: u64 = 10_000_000_000;
pub const MAX_CONNECTIONS: usize = 64;
pub const HASH_LEN: usize = 32;
pub const X25519_LEN: usize = 32;
pub const P256_POINT_LEN: usize = 65;

pub open spec fn request_end(t: u64) -> int {
    if t as int + REQUEST_NS > u64::MAX { u64::MAX as int }
    else { t as int + REQUEST_NS }
}
pub open spec fn timeout_spec(elapsed: u64, first: Option<u64>) -> Option<u64> {
    match first {
        None => if elapsed < IDLE_NS { Some((IDLE_NS - elapsed) as u64) } else { None },
        Some(t) => if elapsed >= request_end(t) { None }
            else { Some(if request_end(t) - elapsed < READ_NS {
                (request_end(t) - elapsed) as u64
            } else { READ_NS }) },
    }
}
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
// Applies to every read in an arbitrary connection: each scheduled expiration
// lies within the same fixed budget. Event delivery by the OS is not modeled.
pub proof fn deadline_bound(elapsed: u64, first: Option<u64>, d: u64)
    requires timeout_spec(elapsed, first) == Some(d),
    ensures elapsed as int + d <= match first { None => IDLE_NS as int,
        Some(t) => t as int + REQUEST_NS },
        d > 0,
{}

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
pub open spec fn admission_result(active: usize) -> Option<usize> {
    if active < MAX_CONNECTIONS { Some((active + 1) as usize) } else { None }
}
pub open spec fn release_result(active: usize) -> usize {
    if active == 0 { 0 } else { (active - 1) as usize }
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
pub proof fn admission_round_trip(active: usize)
    requires active < MAX_CONNECTIONS,
    ensures admission_result(active) is Some,
        release_result(admission_result(active)->Some_0) == active,
{}

pub open spec fn port_start(s: Seq<u8>) -> int {
    if s.len() > 0 && s[0] == 43 { 1 } else { 0 }
}
pub open spec fn digit_range(s: Seq<u8>, a: int, b: int) -> bool {
    forall|i: int| a <= i < b ==> 48 <= #[trigger] s[i] <= 57
}
pub open spec fn port_form(s: Seq<u8>, p: int) -> bool {
    let a = port_start(s);
    a < s.len() && digit_range(s, a, s.len() as int)
        && crate::decimal_value(s.subrange(a, s.len() as int)) == p
}
proof fn decimal_prefix(s: Seq<u8>, a: int, i: int)
    requires 0 <= a <= i <= s.len(), digit_range(s, a, s.len() as int),
    ensures crate::decimal_value(s.subrange(a, i)) <= crate::decimal_value(s.subrange(a, s.len() as int)),
    decreases s.len() - i,
{
    if i < s.len() {
        assert(s.subrange(a, i+1).subrange(0, i-a) =~= s.subrange(a, i));
        assert(s.subrange(a, i+1).last() == s[i]);
        decimal_prefix(s, a, i+1);
    }
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
pub proof fn sizes_agree()
    ensures crate::aead::KEY_LEN == HASH_LEN, X25519_LEN == 32, P256_POINT_LEN == 65,
{}
}
