use vstd::prelude::*;
// The original parser source is shared unchanged, including its contracts.
pub mod aead;
pub mod record;
pub mod app;
pub mod io;
#[path = "../../http-subset/src/lib.rs"]
pub mod http;
use http::{parse_policy, Policy, Request, MAX_REQUEST};

verus! {
#[derive(Debug, PartialEq, Eq)]
pub enum Reason { Grammar, Size, Slow }
pub enum Event<'a> { Bytes(&'a [u8]), ReadTimeout, Eof, IdleExpired }
#[derive(Debug, PartialEq, Eq)]
pub enum Action { ReadMore, Dispatch(Request), Reject(Reason), CloseSilently, Done }

pub struct Machine {
    pub bytes: Vec<u8>,
    pub terminal: bool,
    pub steps: usize,
    pub policy: Policy,
}
impl Machine {
    pub open spec fn valid(&self) -> bool {
        self.policy.valid() && self.bytes.len() <= MAX_REQUEST + 1
        && self.steps <= MAX_REQUEST + 2
        && (!self.terminal ==> self.steps <= self.bytes.len() <= MAX_REQUEST
            && http::request_spec(self.bytes@, &self.policy).is_none())
    }
    pub open spec fn rank(&self) -> int {
        if self.terminal { 0 } else { MAX_REQUEST + 2 - self.steps }
    }
    pub fn new() -> (m: Self)
        ensures m.valid(), !m.terminal, m.steps == 0, m.bytes@ == Seq::<u8>::empty(),
    { Self::with_policy(Policy::builtin()) }
    pub fn with_policy(policy: Policy) -> (m: Self)
        requires policy.valid(),
        ensures m.valid(), !m.terminal, m.steps == 0, m.bytes@ == Seq::<u8>::empty(), m.policy == policy,
    { Self { bytes: Vec::new(), terminal: false, steps: 0, policy } }
    pub fn bytes(&self) -> (r: &[u8]) ensures r@ == self.bytes@ { &self.bytes }
    pub fn terminal(&self) -> (r: bool) ensures r == self.terminal { self.terminal }
    pub fn steps(&self) -> (r: usize) ensures r == self.steps { self.steps }

    pub fn step(&mut self, event: Event) -> (a: Action)
        requires old(self).valid(),
        ensures final(self).valid(), final(self).policy == old(self).policy,
            old(self).terminal ==> final(self).terminal && final(self).bytes@ == old(self).bytes@
                && final(self).steps == old(self).steps && a is Done,
            !old(self).terminal ==> final(self).steps == old(self).steps + 1
                && final(self).rank() < old(self).rank(),
            final(self).bytes.len() >= old(self).bytes.len(),
            match a {
                Action::ReadMore => !final(self).terminal && final(self).bytes.len() > old(self).bytes.len(),
                Action::Dispatch(q) => !old(self).terminal && final(self).terminal
                    && http::request_spec(final(self).bytes@, &final(self).policy) == Some((q.head, q.path_start as int, q.path_end as int)),
                Action::Reject(r) => !old(self).terminal && final(self).terminal && match r {
                    Reason::Size => final(self).bytes.len() == MAX_REQUEST + 1,
                    Reason::Slow => final(self).bytes.len() > 0 && (event is ReadTimeout || event is IdleExpired),
                    Reason::Grammar => http::request_spec(final(self).bytes@, &final(self).policy).is_none(),
                },
                Action::CloseSilently => !old(self).terminal && final(self).terminal && final(self).bytes.len() == 0
                    && (event is ReadTimeout || event is IdleExpired),
                Action::Done => old(self).terminal,
            },
            !old(self).terminal && (event is ReadTimeout || event is IdleExpired)
                ==> (if old(self).bytes.len() == 0 { a is CloseSilently }
                    else { a == Action::Reject(Reason::Slow) }),
            !old(self).terminal && event is Eof ==> a == Action::Reject(Reason::Grammar),
            !old(self).terminal && event is Bytes
                && event->Bytes_0.len() > MAX_REQUEST - old(self).bytes.len()
                ==> a == Action::Reject(Reason::Size),
    {
        if self.terminal { return Action::Done; }
        self.steps += 1;
        self.terminal = true;
        match event {
            Event::ReadTimeout | Event::IdleExpired => {
                if self.bytes.len() == 0 { Action::CloseSilently }
                else { Action::Reject(Reason::Slow) }
            },
            Event::Eof => {
                // Every active state has already failed a complete parse.
                let q = parse_policy(self.bytes.as_slice(), &self.policy);
                match q {
                    Some(q) => Action::Dispatch(q),
                    None => Action::Reject(Reason::Grammar),
                }
            },
            Event::Bytes(chunk) => {
                let room = MAX_REQUEST + 1 - self.bytes.len();
                let count = if chunk.len() < room { chunk.len() } else { room };
                append(&mut self.bytes, &chunk[..count]);
                if self.bytes.len() > MAX_REQUEST { return Action::Reject(Reason::Size); }
                match parse_policy(self.bytes.as_slice(), &self.policy) {
                    Some(q) => Action::Dispatch(q),
                    None => {
                        if count == 0 { return Action::Reject(Reason::Grammar); }
                        if complete(self.bytes.as_slice()) { return Action::Reject(Reason::Grammar); }
                        self.terminal = false;
                        Action::ReadMore
                    },
                }
            },
        }
    }
}

fn append(buf: &mut Vec<u8>, input: &[u8])
    requires old(buf).len() + input.len() <= usize::MAX,
    ensures final(buf)@ == old(buf)@ + input@,
{
    let ghost before = buf@;
    let mut i = 0;
    while i < input.len()
        invariant i <= input.len(), before.len() + input.len() <= usize::MAX,
            buf@ == before + input@.subrange(0, i as int),
        decreases input.len() - i,
    { buf.push(input[i]); i += 1; }
}
fn complete(bytes: &[u8]) -> (r: bool)
    requires bytes.len() <= MAX_REQUEST,
{
    let mut j = 0;
    while j + 3 < bytes.len()
        invariant j <= bytes.len() <= MAX_REQUEST,
        decreases bytes.len() - j,
    {
        if bytes[j] == 13 && bytes[j+1] == 10 && bytes[j+2] == 13 && bytes[j+3] == 10 { return true; }
        j += 1;
    }
    false
}

#[derive(Clone, Copy)]
pub enum Status { Ok, NotFound, BadRequest }
pub open spec fn status_line(status: Status) -> Seq<u8> {
    match status {
        Status::Ok => b"HTTP/1.1 200 OK\r\n"@,
        Status::NotFound => b"HTTP/1.1 404 Not Found\r\n"@,
        Status::BadRequest => b"HTTP/1.1 400 Bad Request\r\n"@,
    }
}
pub open spec fn digits(n: nat) -> Seq<u8>
    decreases n,
{
    if n < 10 { seq![(48 + n) as u8] }
    else { digits(n / 10) + seq![(48 + n % 10) as u8] }
}
pub open spec fn decimal_value(s: Seq<u8>) -> nat
    decreases s.len(),
{
    if s.len() == 0 { 0 }
    else { 10 * decimal_value(s.subrange(0, s.len()-1)) + (s.last() - 48) as nat }
}
fn decimal(n: usize) -> (out: Vec<u8>)
    ensures out@ == digits(n as nat), decimal_value(out@) == n,
        out.len() <= n / 10 + 2,
        forall|i: int| 0 <= i < out.len() ==> 48 <= #[trigger] out@[i] <= 57,
    decreases n,
{
    let mut out = if n < 10 { Vec::new() } else { decimal(n / 10) };
    let ghost prefix = out@;
    out.push(48 + (n % 10) as u8);
    assert(out@.subrange(0, out.len()-1) =~= prefix);
    out
}
// Arbitrary body lengths are supported. Only an unrepresentable combined
// buffer length is rejected; HEAD never needs to copy the body.
pub fn response(status: Status, body: &[u8], head: bool) -> (result: Option<Vec<u8>>)
    ensures match result {
        Some(out) => out@ == status_line(status) + b"Content-Length: "@ + digits(body.len() as nat)
            + b"\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n"@
            + (if head { Seq::<u8>::empty() } else { body@ }),
        None => !head && status_line(status).len() + b"Content-Length: "@.len()
            + digits(body.len() as nat).len()
            + b"\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n"@.len()
            + body.len() > usize::MAX,
    },
        decimal_value(digits(body.len() as nat)) == body.len(),
        forall|i: int| 0 <= i < digits(body.len() as nat).len() ==>
            48 <= #[trigger] digits(body.len() as nat)[i] <= 57,
{
    let mut out = Vec::new();
    match status {
        Status::Ok => append(&mut out, b"HTTP/1.1 200 OK\r\n"),
        Status::NotFound => append(&mut out, b"HTTP/1.1 404 Not Found\r\n"),
        Status::BadRequest => append(&mut out, b"HTTP/1.1 400 Bad Request\r\n"),
    }
    append(&mut out, b"Content-Length: ");
    let length = decimal(body.len());
    append(&mut out, length.as_slice());
    append(&mut out, b"\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n");
    if !head {
        if body.len() > usize::MAX - out.len() { return None; }
        append(&mut out, body);
    }
    Some(out)
}

// Induction over arbitrary traces satisfying step's transition contract.
// 4098 events means 4099 snapshots, including the initial state.
pub proof fn event_bound(trace: Seq<Machine>)
    requires trace.len() == MAX_REQUEST + 3,
        trace[0].steps == 0,
        forall|i: int| 0 <= i < trace.len() ==> #[trigger] trace[i].valid(),
        forall|i: int| 0 <= i < trace.len()-1 ==> (
            (#[trigger] trace[i].terminal ==> trace[i+1].terminal)
            && (!trace[i].terminal ==> trace[i+1].steps == trace[i].steps + 1)),
    ensures trace[MAX_REQUEST + 2].terminal,
{
    trace_prefix(trace, MAX_REQUEST + 2);
    assert(trace[MAX_REQUEST + 2].valid());
}
proof fn trace_prefix(trace: Seq<Machine>, i: int)
    requires 0 <= i < trace.len(), trace[0].steps == 0,
        forall|j: int| 0 <= j < trace.len()-1 ==> (
            (#[trigger] trace[j].terminal ==> trace[j+1].terminal)
            && (!trace[j].terminal ==> trace[j+1].steps == trace[j].steps + 1)),
    ensures trace[i].terminal || trace[i].steps == i,
    decreases i,
{
    if i > 0 { trace_prefix(trace, i-1); }
}
}
