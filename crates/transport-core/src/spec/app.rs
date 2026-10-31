use super::*;

verus! {
pub open spec fn route_status_spec(r: Route) -> Status {
    match r { Route::Health | Route::Root => Status::Ok, Route::NotFound => Status::NotFound }
}

pub open spec fn code(s: Status) -> u16 {
    match s { Status::Ok => 200, Status::NotFound => 404, Status::BadRequest => 400 }
}

// Both the log code and response formatter consume this single selected status.
pub proof fn status_agrees(s: Status)
    ensures (code(s) == 200 ==> status_line(s) == b"HTTP/1.1 200 OK\r\n"@),
        (code(s) == 404 ==> status_line(s) == b"HTTP/1.1 404 Not Found\r\n"@),
        (code(s) == 400 ==> status_line(s) == b"HTTP/1.1 400 Bad Request\r\n"@),
{}

pub open spec fn safe(c: u8) -> bool {
    b'a' <= c <= b'z' || b'A' <= c <= b'Z' || b'0' <= c <= b'9' || c == b'-'
}

// Independent filter followed by take, preserving original order.
pub open spec fn filtered(s: Seq<u8>) -> Seq<u8>
    decreases s.len(),
{
    if s.len() == 0 { Seq::empty() }
    else { filtered(s.drop_last()) + (if safe(s.last()) { seq![s.last()] } else { Seq::empty() }) }
}

pub open spec fn sanitize_spec(s: Seq<u8>) -> Seq<u8> {
    filtered(s).subrange(0, if filtered(s).len() < 64 { filtered(s).len() as int } else { 64 })
}

pub open spec fn crlf(s: Seq<u8>, i: int) -> bool {
    0 <= i && i + 1 < s.len() && s[i] == 13 && s[i+1] == 10
}

pub open spec fn line_end(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    if at < s.len() && !crlf(s, at) { line_end(s, at + 1) } else { at }
}

pub open spec fn colon_end(s: Seq<u8>, at: int, end: int) -> int
    recommends 0 <= at <= end <= s.len(),
    decreases end - at,
{
    if at < end && s[at] != b':' { colon_end(s, at+1, end) } else { at }
}

// Stop is the first empty diagnostic line, or the end if no such line exists.
pub open spec fn stop(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    let e = line_end(s, at);
    if 0 <= at < e && e+2 <= s.len() { stop(s, e+2) } else { e }
}

pub(super) proof fn line_bounds(s: Seq<u8>, at: int)
    requires 0 <= at <= s.len(),
    ensures at <= line_end(s, at) <= s.len(),
        line_end(s, at) < s.len() ==> crlf(s, line_end(s, at)),
    decreases s.len() - at,
{
    if at < s.len() && !crlf(s, at) { line_bounds(s, at+1); }
}

pub(super) proof fn stop_bounds(s: Seq<u8>, at: int)
    requires 0 <= at <= s.len(),
    ensures line_end(s, at) <= stop(s, at) <= s.len(), at <= stop(s, at),
    decreases s.len() - at,
{
    line_bounds(s, at);
    let e = line_end(s, at);
    if e < s.len() && e != at { stop_bounds(s, e+2); }
}

pub open spec fn next_from(s: Seq<u8>, at: int, pos: int) -> Option<(int, int, int)>
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    let e = line_end(s, at);
    if at < 0 || e <= at || e+2 > s.len() { None }
    else {
        let c = colon_end(s, at, e);
        if at >= pos && at <= c < e { Some((at, c, e+2)) }
        else { next_from(s, e+2, pos) }
    }
}

pub open spec fn next_spec(s: Seq<u8>, pos: int) -> Option<(int, int, int)> {
    let first = line_end(s, 0);
    if first < s.len() { next_from(s, first+2, pos) } else { None }
}

pub open spec fn diagnostic_valid(d: Diagnostic, s: Seq<u8>) -> bool {
    match d {
        Diagnostic::TooManyHeaders(a,b) | Diagnostic::HardReject(a,b) | Diagnostic::Duplicate(a,b) |
        Diagnostic::Guarded(a,b) | Diagnostic::Unknown(a,b) => a <= b <= s.len(),
        _ => true,
    }
}

pub open spec fn decision(class: u8, host: bool, connection: bool, p: &Policy,
    count: int, a: usize, b: usize) -> Option<Diagnostic> {
    if count > 24 { Some(Diagnostic::TooManyHeaders(a,b)) }
    else if 19 <= class < 28 { Some(Diagnostic::HardReject(a,b)) }
    else if class == 0 && host || class == 2 && connection { Some(Diagnostic::Duplicate(a,b)) }
    else if !http::drops(p, class) { Some(if class < 44 { Diagnostic::Guarded(a,b) }
        else { Diagnostic::Unknown(a,b) }) }
    else { None }
}

pub open spec fn diagnose_from(s: Seq<u8>, p: &Policy, pos: int,
    count: int, host: bool, connection: bool) -> Diagnostic
    decreases s.len() - pos,
{
    match next_spec(s, pos) {
        Some((a,b,n)) => {
            if 0 <= a <= b <= s.len() && pos < n <= s.len() {
                let class = http::class_spec(s.subrange(a,b));
                match decision(class, host, connection, p, count+1, a as usize, b as usize) {
                    Some(d) => d,
                    None => diagnose_from(s,p,n,count+1,host || class == 0,connection || class == 2),
                }
            } else { Diagnostic::Grammar }
        },
        None => Diagnostic::Grammar,
    }
}

pub open spec fn prefix(d: Diagnostic) -> Seq<u8> {
    match d {
        Diagnostic::TooManyHeaders(_,_) => b"status=400 reason=too_many_headers"@,
        Diagnostic::HardReject(_,_) => b"status=400 reason=hard_reject name="@,
        Diagnostic::Duplicate(_,_) => b"status=400 reason=duplicate name="@,
        Diagnostic::Guarded(_,_) => b"status=400 reason=guarded name="@,
        Diagnostic::Unknown(_,_) => b"status=400 reason=unknown_header name="@,
        Diagnostic::Grammar => b"status=400 reason=grammar"@,
    }
}

pub open spec fn name_span(d: Diagnostic) -> Option<(usize,usize)> {
    match d {
        Diagnostic::HardReject(a,b) | Diagnostic::Duplicate(a,b) |
        Diagnostic::Guarded(a,b) | Diagnostic::Unknown(a,b) => Some((a,b)),
        _ => None,
    }
}

pub open spec fn diagnostic_spec(d: Diagnostic, s: Seq<u8>) -> Seq<u8> {
    prefix(d) + match name_span(d) {
        Some((a,b)) => sanitize_spec(s.subrange(a as int,b as int)), None => Seq::empty(),
    }
}

pub open spec fn folded(s: Seq<u8>) -> Seq<u8> {
    Seq::new(s.len(), |i: int| http::ascii_lower(s[i]))
}

pub open spec fn live(time: u64, now: u64) -> bool {
    now < time || now - time < 60000
}

pub open spec fn retained(e: Seq<(Vec<u8>,u64)>, now: u64) -> Seq<(Vec<u8>,u64)>
    decreases e.len(),
{
    if e.len() == 0 { Seq::empty() }
    else { retained(e.drop_last(),now) + if live(e.last().1,now) { seq![e.last()] } else { Seq::empty() } }
}

pub open spec fn cap(limit: u16) -> int { if limit < 64 { limit as int } else { 64 } }

pub open spec fn present(e: Seq<(Vec<u8>,u64)>, name: Seq<u8>) -> bool {
    exists|i: int| 0 <= i < e.len() && e[i].0@ == folded(name)
}

impl UnknownLog {
    pub open spec fn valid(&self, limit: u16) -> bool {
        self.entries.len() <= cap(limit) &&
        forall|i: int, j: int| 0 <= i < self.entries.len() && 0 <= j < self.entries@[i].0.len()
            ==> !(b'A' <= #[trigger] self.entries@[i].0@[j] <= b'Z')
    }
}

// Number of entries timestamped at or after a window's left edge.
pub open spec fn recent(e: Seq<(Vec<u8>,u64)>, start: u64) -> nat
    decreases e.len(),
{
    if e.len() == 0 { 0 } else {
        recent(e.drop_last(),start) + if e.last().1 >= start { 1nat } else { 0nat }
    }
}

pub(super) proof fn recent_bounds(e: Seq<(Vec<u8>,u64)>, start: u64)
    ensures recent(e,start) <= e.len(),
    decreases e.len(),
{
    if e.len() > 0 { recent_bounds(e.drop_last(),start); }
}

// An entry created in this half-open 60-second window cannot be pruned inside it.
pub(super) proof fn retain_recent(e: Seq<(Vec<u8>,u64)>, start: u64, now: u64)
    requires start <= now, now - start < 60000,
    ensures recent(retained(e,now),start) == recent(e,start),
    decreases e.len(),
{
    if e.len() > 0 {
        retain_recent(e.drop_last(),start,now);
        if live(e.last().1,now) {
            assert(retained(e,now).drop_last() =~= retained(e.drop_last(),now));
        }
    }
}

pub open spec fn emitted_count(emit: Seq<bool>) -> nat
    decreases emit.len(),
{
    if emit.len() == 0 { 0 } else { emitted_count(emit.drop_last()) + if emit.last() { 1nat } else { 0nat } }
}

pub open spec fn log_transition(before: UnknownLog, after: UnknownLog, now: u64, emit: bool) -> bool {
    if emit {
        after.entries.len() == retained(before.entries@,now).len()+1
        && after.entries@.drop_last() == retained(before.entries@,now)
        && after.entries@.last().1 == now
    } else { after.entries@ == retained(before.entries@,now) }
}

pub(super) proof fn window_accounting(states: Seq<UnknownLog>, times: Seq<u64>, emit: Seq<bool>, start: u64)
    requires states.len() == times.len()+1, emit.len() == times.len(),
        forall|i: int| 0 <= i < times.len() ==> start <= #[trigger] times[i] && times[i] - start < 60000,
        forall|i: int| 0 <= i < times.len() ==> log_transition(states[i],states[i+1],times[i],emit[i]),
    ensures recent(states.last().entries@,start)
        == recent(states[0].entries@,start) + emitted_count(emit),
    decreases times.len(),
{
    if times.len() > 0 {
        let k = times.len()-1;
        window_accounting(states.drop_last(),times.drop_last(),emit.drop_last(),start);
        retain_recent(states[k].entries@,start,times[k]);
        assert(log_transition(states[k],states[k+1],times[k],emit[k]));
    }
}

// Arbitrarily long traces, arbitrary initial state and any half-open 60s window.
// Applying to the contiguous calls inside any window of a non-decreasing clock
// establishes the bound for every sliding minute, not only aligned buckets.
pub proof fn sliding_window_bound(states: Seq<UnknownLog>, times: Seq<u64>, emit: Seq<bool>,
    start: u64, limit: u16)
    requires states.len() == times.len()+1, emit.len() == times.len(),
        forall|i: int| 0 <= i < states.len() ==> #[trigger] states[i].valid(limit),
        forall|i: int| 0 <= i < times.len() ==> start <= #[trigger] times[i] && times[i] - start < 60000,
        forall|i: int| 0 <= i < times.len() ==> log_transition(states[i],states[i+1],times[i],emit[i]),
    ensures emitted_count(emit) <= cap(limit),
{
    window_accounting(states,times,emit,start);
    recent_bounds(states.last().entries@,start);
    assert(states[states.len()-1].valid(limit));
}
}
