use super::*;

verus! {
impl Machine {
    pub open spec fn valid(&self) -> bool {
        self.policy.valid() && self.bytes.len() <= MAX_REQUEST + 1
        && self.steps <= MAX_REQUEST + 2
        && (!self.terminal ==> self.steps <= self.bytes.len() <= MAX_REQUEST
            && http::request_spec(self.bytes@, &self.policy).is_none())
    }
}

impl Machine {
    pub open spec fn rank(&self) -> int {
        if self.terminal { 0 } else { MAX_REQUEST + 2 - self.steps }
    }
}

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

pub(super) proof fn trace_prefix(trace: Seq<Machine>, i: int)
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
