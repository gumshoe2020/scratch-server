//! Pure post-machine decisions. No socket, clock, lock, filesystem or stdout access.
use vstd::prelude::*;
use super::*;
use super::http::{Policy, Request, classify, drop_header};

verus! {
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Route { Health, Root, NotFound }
pub fn same(a: &[u8], b: &[u8]) -> (r: bool)
    ensures r == (a@ == b@),
{
    if a.len() != b.len() { return false; }
    let mut i = 0;
    while i < a.len()
        invariant i <= a.len(), a.len() == b.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.len() - i,
    {
        if a[i] != b[i] { return false; }
        i += 1;
    }
    assert(a@ =~= b@);
    true
}
pub fn route(path: &[u8]) -> (r: Route)
    ensures (r is Health) == (path@ == b"/health"@),
        (r is Root) == (path@ == b"/"@),
        (r is NotFound) == (path@ != b"/health"@ && path@ != b"/"@),
{
    proof { reveal_byteslit(b"/health"); reveal_byteslit(b"/");
        assert(b"/health"@.len() == 7); assert(b"/"@.len() == 1);
    }
    if same(path, b"/health") { Route::Health }
    else if same(path, b"/") { Route::Root } else { Route::NotFound }
}
pub open spec fn route_status_spec(r: Route) -> Status {
    match r { Route::Health | Route::Root => Status::Ok, Route::NotFound => Status::NotFound }
}
pub fn route_status(r: Route) -> (s: Status) ensures s == route_status_spec(r) {
    match r { Route::Health | Route::Root => Status::Ok, Route::NotFound => Status::NotFound }
}
pub open spec fn code(s: Status) -> u16 {
    match s { Status::Ok => 200, Status::NotFound => 404, Status::BadRequest => 400 }
}
pub fn status_code(s: Status) -> (n: u16) ensures n == code(s) {
    match s { Status::Ok => 200, Status::NotFound => 404, Status::BadRequest => 400 }
}
// Both the log code and response formatter consume this single selected status.
pub proof fn status_agrees(s: Status)
    ensures (code(s) == 200 ==> status_line(s) == b"HTTP/1.1 200 OK\r\n"@),
        (code(s) == 404 ==> status_line(s) == b"HTTP/1.1 404 Not Found\r\n"@),
        (code(s) == 400 ==> status_line(s) == b"HTTP/1.1 400 Bad Request\r\n"@),
{}

pub enum Task { ReadMore, Dispatch(Request), Reject(Reason), CloseSilently, Done }
pub struct Plan { pub task: Task, pub handler_increment: usize }
pub fn plan(a: Action) -> (p: Plan)
    ensures p.handler_increment == (if a is Dispatch { 1int } else { 0int }),
        match a {
            Action::ReadMore => p.task is ReadMore,
            Action::Dispatch(q) => p.task == Task::Dispatch(q),
            Action::Reject(r) => p.task == Task::Reject(r),
            Action::CloseSilently => p.task is CloseSilently,
            Action::Done => p.task is Done,
        },
{
    let (task, handler_increment) = match a {
        Action::ReadMore => (Task::ReadMore, 0),
        Action::Dispatch(q) => (Task::Dispatch(q), 1),
        Action::Reject(r) => (Task::Reject(r), 0),
        Action::CloseSilently => (Task::CloseSilently, 0),
        Action::Done => (Task::Done, 0),
    };
    Plan { task, handler_increment }
}
// The result is None for all tasks that must not write, including ReadMore.
pub fn task_response(task: &Task, r: Route, root: &[u8]) -> (out: Option<Vec<u8>>)
    ensures match task {
        Task::Dispatch(q) => match out {
            Some(v) => v@ == status_line(route_status_spec(r)) + b"Content-Length: "@
                + super::digits((match r { Route::Health => 2nat, Route::Root => root.len() as nat,
                    Route::NotFound => 10nat }))
                + b"\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n"@
                + (if q.head { Seq::empty() } else { match r {
                    Route::Health => b"ok"@, Route::Root => root@, Route::NotFound => b"not found\n"@ } }),
            None => r is Root && !q.head,
        },
        Task::Reject(_) => out is Some && out->Some_0@ == status_line(Status::BadRequest)
            + b"Content-Length: 12\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nbad request\n"@,
        _ => out is None,
    },
{
    proof {
        reveal_byteslit(b"ok"); reveal_byteslit(b"not found\n");
        reveal_byteslit(b"bad request\n");
        reveal_byteslit(b"12");
        reveal_byteslit(b"Content-Length: 12\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nbad request\n");
        reveal_byteslit(b"Content-Length: ");
        reveal_byteslit(b"\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n");
        reveal_with_fuel(super::digits, 3);
        assert(super::digits(12) =~= b"12"@);
        assert(b"Content-Length: "@ + b"12"@ + b"\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n"@ + b"bad request\n"@
            =~= b"Content-Length: 12\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nbad request\n"@);
    }
    match task {
        Task::Dispatch(q) => {
            let s = route_status(r);
            match r { Route::Health => response(s, b"ok", q.head),
                Route::Root => response(s, root, q.head),
                Route::NotFound => response(s, b"not found\n", q.head) }
        },
        Task::Reject(_) => response(Status::BadRequest, b"bad request\n", false),
        _ => None,
    }
}

pub open spec fn safe(c: u8) -> bool {
    b'a' <= c <= b'z' || b'A' <= c <= b'Z' || b'0' <= c <= b'9' || c == b'-'
}
fn is_safe(c: u8) -> (r: bool) ensures r == safe(c) {
    c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z' || c >= b'0' && c <= b'9' || c == b'-'
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
pub fn sanitize(name: &[u8]) -> (out: Vec<u8>)
    ensures out@ == sanitize_spec(name@), out.len() <= 64,
        forall|i: int| 0 <= i < out.len() ==> safe(#[trigger] out@[i]),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < name.len()
        invariant i <= name.len(), out@ == sanitize_spec(name@.subrange(0, i as int)),
            out.len() <= 64,
            forall|j: int| 0 <= j < out.len() ==> safe(#[trigger] out@[j]),
        decreases name.len() - i,
    {
        proof {
            assert(name@.subrange(0, i + 1).drop_last() =~= name@.subrange(0, i as int));
        }
        if is_safe(name[i]) && out.len() < 64 { out.push(name[i]); }
        i += 1;
    }
    assert(name@.subrange(0, name.len() as int) =~= name@);
    out
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
fn scan_line(s: &[u8], at: usize) -> (end: usize)
    requires at <= s.len(),
    ensures at <= end <= s.len(), end as int == line_end(s@, at as int),
        end < s.len() ==> crlf(s@, end as int),
        forall|i: int| at <= i < end ==> !crlf(s@, i),
{
    let mut i = at;
    while i < s.len()
        invariant at <= i <= s.len(), line_end(s@, at as int) == line_end(s@, i as int),
            forall|j: int| at <= j < i ==> !crlf(s@, j),
        decreases s.len() - i,
    {
        if s.len() - i >= 2 && s[i] == 13 && s[i+1] == 10 { return i; }
        i += 1;
    }
    i
}
pub open spec fn colon_end(s: Seq<u8>, at: int, end: int) -> int
    recommends 0 <= at <= end <= s.len(),
    decreases end - at,
{
    if at < end && s[at] != b':' { colon_end(s, at+1, end) } else { at }
}
fn scan_colon(s: &[u8], at: usize, end: usize) -> (c: usize)
    requires at <= end <= s.len(),
    ensures at <= c <= end, c as int == colon_end(s@, at as int, end as int),
        c < end ==> s@[c as int] == b':',
        forall|i: int| at <= i < c ==> s[i] != b':',
{
    let mut i = at;
    while i < end
        invariant at <= i <= end <= s.len(),
            colon_end(s@, at as int, end as int) == colon_end(s@, i as int, end as int),
            forall|j: int| at <= j < i ==> s[j] != b':',
        decreases end - i,
    {
        if s[i] == b':' { return i; }
        i += 1;
    }
    i
}
// Stop is the first empty diagnostic line, or the end if no such line exists.
pub open spec fn stop(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    let e = line_end(s, at);
    if 0 <= at < e && e+2 <= s.len() { stop(s, e+2) } else { e }
}
proof fn line_bounds(s: Seq<u8>, at: int)
    requires 0 <= at <= s.len(),
    ensures at <= line_end(s, at) <= s.len(),
        line_end(s, at) < s.len() ==> crlf(s, line_end(s, at)),
    decreases s.len() - at,
{
    if at < s.len() && !crlf(s, at) { line_bounds(s, at+1); }
}
proof fn stop_bounds(s: Seq<u8>, at: int)
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
// Rescan from the request line so even arbitrary caller cursors cannot jump past
// an empty line. Production cursors are 0, then the preceding result's next.
pub fn next_header_name(s: &[u8], pos: usize) -> (r: Option<(usize, usize, usize)>)
    ensures match r {
        Some((a,b,n)) => next_spec(s@, pos as int) == Some((a as int,b as int,n as int))
            && line_end(s@, 0) + 2 <= a <= b < n <= s.len() && n > pos
            && b <= stop(s@, line_end(s@, 0)+2)
            && (forall|i: int| a <= i < b ==> s[i] != b':' && !crlf(s@, i)),
        None => next_spec(s@, pos as int).is_none(),
    },
{
    let first = scan_line(s, 0);
    if first == s.len() { return None; }
    let mut at = first + 2;
    let ghost begin = at;
    proof { stop_bounds(s@, begin as int); }
    while at < s.len()
        invariant begin == first + 2, first == line_end(s@, 0),
            begin <= at <= s.len(),
            next_spec(s@, pos as int) == next_from(s@, at as int, pos as int),
            stop(s@, begin as int) == stop(s@, at as int),
        decreases s.len() - at,
    {
        let end = scan_line(s, at);
        if end == s.len() || end == at { return None; }
        proof { stop_bounds(s@, at as int); }
        let c = scan_colon(s, at, end);
        if at >= pos && c < end { return Some((at, c, end+2)); }
        at = end + 2;
    }
    None
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Diagnostic {
    TooManyHeaders(usize, usize), HardReject(usize, usize), Duplicate(usize, usize),
    Guarded(usize, usize), Unknown(usize, usize), Grammar,
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
pub fn diagnose(s: &[u8], p: &Policy) -> (d: Diagnostic)
    ensures d == diagnose_from(s@, p, 0, 0, false, false), diagnostic_valid(d, s@),
{
    let mut pos = 0;
    let mut count = 0;
    let mut host = false;
    let mut connection = false;
    loop
        invariant pos <= s.len(), count <= 24,
            diagnose_from(s@,p,0,0,false,false) == diagnose_from(s@,p,pos as int,count as int,host,connection),
        decreases s.len() - pos,
    {
        match next_header_name(s,pos) {
            None => return Diagnostic::Grammar,
            Some((a,b,n)) => {
                count += 1;
                if count > 24 { return Diagnostic::TooManyHeaders(a,b); }
                let class = classify(&s[a..b]);
                if class >= 19 && class < 28 { return Diagnostic::HardReject(a,b); }
                if class == 0 && host || class == 2 && connection { return Diagnostic::Duplicate(a,b); }
                if !drop_header(p,class) {
                    return if class < 44 { Diagnostic::Guarded(a,b) } else { Diagnostic::Unknown(a,b) };
                }
                host = host || class == 0;
                connection = connection || class == 2;
                pos = n;
            },
        }
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
pub fn diagnostic_line(d: Diagnostic, s: &[u8]) -> (out: Vec<u8>)
    requires diagnostic_valid(d,s@),
    ensures out@ == diagnostic_spec(d,s@), out.len() <= 128,
        forall|i: int| 0 <= i < out.len() ==> 32 <= #[trigger] out@[i] <= 126,
{
    let p: &[u8] = match d {
        Diagnostic::TooManyHeaders(_,_) => b"status=400 reason=too_many_headers",
        Diagnostic::HardReject(_,_) => b"status=400 reason=hard_reject name=",
        Diagnostic::Duplicate(_,_) => b"status=400 reason=duplicate name=",
        Diagnostic::Guarded(_,_) => b"status=400 reason=guarded name=",
        Diagnostic::Unknown(_,_) => b"status=400 reason=unknown_header name=",
        Diagnostic::Grammar => b"status=400 reason=grammar",
    };
    proof { assert(p@ == prefix(d)); assert(p.len() <= 64);
        assert forall|i: int| 0 <= i < p.len() implies 32 <= #[trigger] p@[i] <= 126 by {
            reveal_byteslit(b"status=400 reason=too_many_headers");
            reveal_byteslit(b"status=400 reason=hard_reject name=");
            reveal_byteslit(b"status=400 reason=duplicate name=");
            reveal_byteslit(b"status=400 reason=guarded name=");
            reveal_byteslit(b"status=400 reason=unknown_header name=");
            reveal_byteslit(b"status=400 reason=grammar");
        }
    }
    let mut out = Vec::new();
    append(&mut out, p);
    match d {
        Diagnostic::HardReject(a,b) | Diagnostic::Duplicate(a,b) |
        Diagnostic::Guarded(a,b) | Diagnostic::Unknown(a,b) => {
            let name = sanitize(&s[a..b]);
            append(&mut out, name.as_slice());
        },
        _ => (),
    }
    out
}

pub open spec fn folded(s: Seq<u8>) -> Seq<u8> {
    Seq::new(s.len(), |i: int| http::ascii_lower(s[i]))
}
pub fn lowercase(name: &[u8]) -> (out: Vec<u8>)
    ensures out@ == folded(name@),
        forall|i: int| 0 <= i < out.len() ==> !(b'A' <= #[trigger] out@[i] <= b'Z'),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < name.len()
        invariant i <= name.len(), out.len() == i,
            forall|j: int| 0 <= j < i ==> out@[j] == http::ascii_lower(name@[j])
                && !(b'A' <= out@[j] <= b'Z'),
        decreases name.len() - i,
    {
        let c = name[i];
        out.push(if c >= b'A' && c <= b'Z' { c + 32 } else { c });
        i += 1;
    }
    assert(out@ =~= folded(name@));
    out
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
fn prune(e: &mut Vec<(Vec<u8>,u64)>, now: u64)
    requires old(e).len() <= 64,
    ensures final(e)@ == retained(old(e)@,now), final(e).len() <= old(e).len(),
        forall|i: int| 0 <= i < final(e).len() ==> live(#[trigger] final(e)@[i].1,now),
        forall|i: int| 0 <= i < final(e).len() ==> old(e)@.contains(#[trigger] final(e)@[i]),
    decreases old(e).len(),
{
    if e.len() == 0 { return; }
    let ghost before = e@;
    let last = e.pop().unwrap();
    prune(e,now);
    if now < last.1 || now - last.1 < 60000 { e.push(last); }
}
pub open spec fn cap(limit: u16) -> int { if limit < 64 { limit as int } else { 64 } }
pub open spec fn present(e: Seq<(Vec<u8>,u64)>, name: Seq<u8>) -> bool {
    exists|i: int| 0 <= i < e.len() && e[i].0@ == folded(name)
}
pub struct UnknownLog { pub entries: Vec<(Vec<u8>,u64)> }
impl UnknownLog {
    pub open spec fn valid(&self, limit: u16) -> bool {
        self.entries.len() <= cap(limit) &&
        forall|i: int, j: int| 0 <= i < self.entries.len() && 0 <= j < self.entries@[i].0.len()
            ==> !(b'A' <= #[trigger] self.entries@[i].0@[j] <= b'Z')
    }
    pub fn new() -> (s: Self)
        ensures forall|limit: u16| s.valid(limit), s.entries.len() == 0,
    { Self { entries: Vec::new() } }
    pub fn should_log(&mut self, name: &[u8], limit: u16, now: u64) -> (yes: bool)
        requires old(self).valid(limit),
        ensures final(self).valid(limit),
            yes ==> final(self).entries.len() == retained(old(self).entries@,now).len()+1
                && final(self).entries@.drop_last() == retained(old(self).entries@,now)
                && final(self).entries@.last().0@ == folded(name@) && final(self).entries@.last().1 == now,
            !yes ==> final(self).entries@ == retained(old(self).entries@,now),
            yes == (retained(old(self).entries@,now).len() < cap(limit)
                && !present(retained(old(self).entries@,now),name@)),
            present(retained(old(self).entries@,now),name@) ==> !yes,
    {
        prune(&mut self.entries, now);
        let bound = if limit < 64 { limit as usize } else { 64 };
        if self.entries.len() >= bound { return false; }
        let lowered = lowercase(name);
        let mut i = 0;
        while i < self.entries.len()
            invariant i <= self.entries.len(), self.valid(limit),
                self.entries@ == retained(old(self).entries@,now), lowered@ == folded(name@),
                forall|j: int| 0 <= j < i ==> self.entries@[j].0@ != lowered@,
            decreases self.entries.len() - i,
        {
            if same(self.entries[i].0.as_slice(), lowered.as_slice()) { return false; }
            i += 1;
        }
        self.entries.push((lowered,now));
        true
    }
}

pub fn needs_root(r: Route) -> (yes: bool) ensures yes == (r is Root) {
    match r { Route::Root => true, _ => false }
}
pub fn rejection_line(s: &[u8], policy: &Policy, reason: &Reason) -> (out: Vec<u8>)
    ensures match reason {
        Reason::Grammar => out@ == diagnostic_spec(diagnose_from(s@,policy,0,0,false,false),s@),
        Reason::Size => out@ == b"status=400 reason=size"@,
        Reason::Slow => out@ == b"status=400 reason=slow"@,
    },
{
    match reason {
        Reason::Grammar => diagnostic_line(diagnose(s,policy),s),
        Reason::Size => { let mut out = Vec::new(); append(&mut out,b"status=400 reason=size"); out },
        Reason::Slow => { let mut out = Vec::new(); append(&mut out,b"status=400 reason=slow"); out },
    }
}
pub fn next_unknown(s: &[u8], pos: usize) -> (r: Option<(usize,usize,usize)>)
    ensures match r {
        Some((a,b,n)) => a <= b <= s.len() && pos < n <= s.len()
            && http::class_spec(s@.subrange(a as int,b as int)) == 44,
        None => true,
    },
{
    let mut cursor = pos;
    loop
        invariant cursor >= pos,
        decreases s.len() - cursor,
    {
        match next_header_name(s,cursor) {
            None => return None,
            Some((a,b,n)) => {
                if classify(&s[a..b]) == 44 { return Some((a,b,n)); }
                cursor = n;
            },
        }
    }
}
pub fn unknown_line(r: Route, name: &[u8]) -> (out: Vec<u8>)
    ensures out@ == (if code(route_status_spec(r)) == 200 {
            b"status=200 note=unknown_header name="@ } else { b"status=404 note=unknown_header name="@ })
            + sanitize_spec(name@),
        out.len() <= 128,
        forall|i: int| 0 <= i < out.len() ==> 32 <= #[trigger] out@[i] <= 126,
{
    let status = route_status(r);
    let p: &[u8] = if status_code(status) == 200 {
        b"status=200 note=unknown_header name="
    } else { b"status=404 note=unknown_header name=" };
    proof {
        reveal_byteslit(b"status=200 note=unknown_header name=");
        reveal_byteslit(b"status=404 note=unknown_header name=");
        assert forall|i: int| 0 <= i < p.len() implies 32 <= #[trigger] p@[i] <= 126 by { }
    }
    let mut out = Vec::new();
    append(&mut out,p);
    let clean = sanitize(name);
    append(&mut out,clean.as_slice());
    out
}

// Number of entries timestamped at or after a window's left edge.
pub open spec fn recent(e: Seq<(Vec<u8>,u64)>, start: u64) -> nat
    decreases e.len(),
{
    if e.len() == 0 { 0 } else {
        recent(e.drop_last(),start) + if e.last().1 >= start { 1nat } else { 0nat }
    }
}
proof fn recent_bounds(e: Seq<(Vec<u8>,u64)>, start: u64)
    ensures recent(e,start) <= e.len(),
    decreases e.len(),
{
    if e.len() > 0 { recent_bounds(e.drop_last(),start); }
}
// An entry created in this half-open 60-second window cannot be pruned inside it.
proof fn retain_recent(e: Seq<(Vec<u8>,u64)>, start: u64, now: u64)
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
proof fn window_accounting(states: Seq<UnknownLog>, times: Seq<u64>, emit: Seq<bool>, start: u64)
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
