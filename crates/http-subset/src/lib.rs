use vstd::prelude::*;

verus! {
pub const MAX_REQUEST: usize = 4096;
pub const MAX_PATH: usize = 256;
pub const MAX_VALUE: usize = 256;
pub const MAX_HEADERS: usize = 24;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub name: u8,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct RawRequest {
    pub head: bool,
    pub path_start: usize,
    pub path_end: usize,
    pub headers: Vec<Header>,
}

pub open spec fn matches(s: Seq<u8>, at: int, text: Seq<u8>) -> bool {
    0 <= at && at + text.len() <= s.len()
        && s.subrange(at, at + text.len()) == text
}

fn equal(s: &[u8], at: usize, text: &[u8]) -> (r: bool)
    ensures r == matches(s@, at as int, text@),
{
    if at > s.len() || text.len() > s.len() - at { return false; }
    let mut i = 0;
    while i < text.len()
        invariant at <= s.len(), text.len() <= s.len() - at,
            i <= text.len(),
            forall|j: int| 0 <= j < i ==> s@[at + j] == text@[j],
        decreases text.len() - i,
    {
        if s[at + i] != text[i] { return false; }
        i += 1;
    }
    assert(s@.subrange(at as int, at + text.len()) =~= text@);
    true
}

// ASCII folding applies only to whitelisted header prefixes, never request syntax.
pub open spec fn ascii_lower(c: u8) -> u8 {
    if b'A' <= c <= b'Z' { (c + 32) as u8 } else { c }
}
fn lower(c: u8) -> (r: u8)
    ensures r == ascii_lower(c),
{
    if c >= b'A' && c <= b'Z' { c + 32 } else { c }
}
pub open spec fn matches_header(s: Seq<u8>, at: int, text: Seq<u8>) -> bool {
    0 <= at && at + text.len() <= s.len()
        && forall|j: int| 0 <= j < text.len() ==>
            ascii_lower(s[at + j]) == ascii_lower(text[j])
}
fn equal_header(s: &[u8], at: usize, text: &[u8]) -> (r: bool)
    ensures r == matches_header(s@, at as int, text@),
{
    if at > s.len() || text.len() > s.len() - at { return false; }
    let mut i = 0;
    while i < text.len()
        invariant at <= s.len(), text.len() <= s.len() - at,
            i <= text.len(),
            forall|j: int| 0 <= j < i ==>
                ascii_lower(s@[at + j]) == ascii_lower(text@[j]),
        decreases text.len() - i,
    {
        if lower(s[at + i]) != lower(text[i]) { return false; }
        i += 1;
    }
    true
}

pub open spec fn path_char(c: u8) -> bool {
    (b'a' <= c <= b'z') || (b'A' <= c <= b'Z') || (b'0' <= c <= b'9')
        || c == b'/' || c == b'-' || c == b'_' || c == b'.'
}
fn is_path(c: u8) -> (r: bool)
    ensures r == path_char(c),
{
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')
        || (c >= b'0' && c <= b'9') || c == b'/' || c == b'-' || c == b'_' || c == b'.'
}

// First byte not in the path alphabet, or the end of the input.
pub open spec fn path_end(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    if at < s.len() && path_char(s[at]) { path_end(s, at + 1) } else { at }
}
fn scan_path(s: &[u8], at: usize) -> (r: usize)
    requires at <= s.len(),
    ensures r == path_end(s@, at as int), at <= r <= s.len(),
        at < s.len() && path_char(s@[at as int]) ==> r > at,
{
    let mut i = at;
    while i < s.len() && is_path(s[i])
        invariant at <= i <= s.len(), path_end(s@, at as int) == path_end(s@, i as int),
        decreases s.len() - i,
    { i += 1; }
    i
}

pub open spec fn value_char(c: u8) -> bool { b' ' <= c <= b'~' }
pub open spec fn value_end(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    if at < s.len() && value_char(s[at]) { value_end(s, at + 1) } else { at }
}
fn scan_value(s: &[u8], at: usize) -> (r: usize)
    requires at <= s.len(),
    ensures r == value_end(s@, at as int), at <= r <= s.len(),
{
    let mut i = at;
    while i < s.len() && s[i] >= b' ' && s[i] <= b'~'
        invariant at <= i <= s.len(), value_end(s@, at as int) == value_end(s@, i as int),
        decreases s.len() - i,
    { i += 1; }
    i
}

pub open spec fn header_prefix(s: Seq<u8>, at: int) -> (u8, int) {
    if matches_header(s, at, b"Host: "@) { (0u8, 6) }
    else if matches_header(s, at, b"Accept: "@) { (1u8, 8) }
    else if matches_header(s, at, b"Connection: "@) { (2u8, 12) }
    else if matches_header(s, at, b"User-Agent: "@) { (3u8, 12) }
    else if matches_header(s, at, b"Accept-Language: "@) { (4u8, 17) }
    else if matches_header(s, at, b"Accept-Encoding: "@) { (5u8, 17) }
    else if matches_header(s, at, b"Upgrade-Insecure-Requests: "@) { (6u8, 27) }
    else if matches_header(s, at, b"Priority: "@) { (7u8, 10) }
    else if matches_header(s, at, b"Referer: "@) { (8u8, 9) }
    else if matches_header(s, at, b"Sec-Fetch-Dest: "@) { (9u8, 16) }
    else if matches_header(s, at, b"Sec-Fetch-Site: "@) { (10u8, 16) }
    else if matches_header(s, at, b"Sec-Fetch-Mode: "@) { (11u8, 16) }
    else if matches_header(s, at, b"Sec-Fetch-User: "@) { (12u8, 16) }
    else if matches_header(s, at, b"Cache-Control: "@) { (13u8, 15) }
    else if matches_header(s, at, b"Sec-GPC: "@) { (14u8, 9) }
    else if matches_header(s, at, b"Sec-CH-UA: "@) { (15u8, 11) }
    else if matches_header(s, at, b"Sec-CH-UA-Mobile: "@) { (16u8, 18) }
    else if matches_header(s, at, b"Sec-CH-UA-Platform: "@) { (17u8, 20) }
    else if matches_header(s, at, b"Pragma: "@) { (18u8, 8) }
    else { (9u8, 0) }
}
fn prefix(s: &[u8], at: usize) -> (r: (u8, usize))
    ensures r.0 == header_prefix(s@, at as int).0,
        r.1 as int == header_prefix(s@, at as int).1,
        r.1 > 0 ==> at + r.1 <= s.len(),
{
    if equal_header(s, at, b"Host: ") { (0, 6) }
    else if equal_header(s, at, b"Accept: ") { (1, 8) }
    else if equal_header(s, at, b"Connection: ") { (2, 12) }
    else if equal_header(s, at, b"User-Agent: ") { (3, 12) }
    else if equal_header(s, at, b"Accept-Language: ") { (4, 17) }
    else if equal_header(s, at, b"Accept-Encoding: ") { (5, 17) }
    else if equal_header(s, at, b"Upgrade-Insecure-Requests: ") { (6, 27) }
    else if equal_header(s, at, b"Priority: ") { (7, 10) }
    else if equal_header(s, at, b"Referer: ") { (8, 9) }
    else if equal_header(s, at, b"Sec-Fetch-Dest: ") { (9, 16) }
    else if equal_header(s, at, b"Sec-Fetch-Site: ") { (10, 16) }
    else if equal_header(s, at, b"Sec-Fetch-Mode: ") { (11, 16) }
    else if equal_header(s, at, b"Sec-Fetch-User: ") { (12, 16) }
    else if equal_header(s, at, b"Cache-Control: ") { (13, 15) }
    else if equal_header(s, at, b"Sec-GPC: ") { (14, 9) }
    else if equal_header(s, at, b"Sec-CH-UA: ") { (15, 11) }
    else if equal_header(s, at, b"Sec-CH-UA-Mobile: ") { (16, 18) }
    else if equal_header(s, at, b"Sec-CH-UA-Platform: ") { (17, 20) }
    else if equal_header(s, at, b"Pragma: ") { (18, 8) }
    else { (9, 0) }
}

pub open spec fn bounded_stage11(h: Header, len: int) -> bool {
    h.name < 19 && 0 <= h.start < h.end <= len && h.end - h.start <= MAX_VALUE
}

// Declarative header grammar, returning exactly the header fields in wire order.
pub open spec fn header_spec(s: Seq<u8>, at: int, remaining: nat) -> Option<Seq<Header>>
    recommends 0 <= at <= s.len(),
    decreases remaining,
{
    if matches(s, at, b"\r\n"@) && at + 2 == s.len() { Some(Seq::empty()) }
    else if remaining == 0 { None }
    else {
        let p = header_prefix(s, at);
        let start = at + p.1;
        let end = value_end(s, start);
        if p.1 == 0 || end <= start || end - start > MAX_VALUE
            || s[start] == b' ' || s[end - 1] == b' '
            || !matches(s, end, b"\r\n"@) { None }
        else {
            match header_spec(s, end + 2, (remaining - 1) as nat) {
                None => None,
                Some(tail) => Some(seq![Header { name: p.0, start: start as usize, end: end as usize }] + tail),
            }
        }
    }
}

fn headers(s: &[u8], at: usize, remaining: usize) -> (r: Option<Vec<Header>>)
    requires s.len() <= MAX_REQUEST, at <= s.len(), remaining <= MAX_HEADERS,
    ensures match r {
        None => header_spec(s@, at as int, remaining as nat).is_none(),
        Some(h) => header_spec(s@, at as int, remaining as nat) == Some(h@)
            && h.len() <= remaining
            && forall|j: int| 0 <= j < h.len() ==> bounded_stage11(#[trigger] h@[j], s.len() as int),
    },
    decreases remaining,
{
    if equal(s, at, b"\r\n") && at + 2 == s.len() { return Some(Vec::new()); }
    if remaining == 0 { return None; }
    let p = prefix(s, at);
    if p.1 == 0 { return None; }
    let start = at + p.1;
    let end = scan_value(s, start);
    if end <= start || end - start > MAX_VALUE || s[start] == b' ' || s[end - 1] == b' '
        || !equal(s, end, b"\r\n") { return None; }
    match headers(s, end + 2, remaining - 1) {
        None => None,
        Some(tail) => {
            let mut result = Vec::new();
            result.push(Header { name: p.0, start, end });
            let mut i = 0;
            while i < tail.len()
                invariant i <= tail.len(), tail.len() < remaining,
                    forall|j: int| 0 <= j < tail.len() ==> bounded_stage11(#[trigger] tail@[j], s.len() as int),
                    forall|j: int| 0 <= j < result.len() ==> bounded_stage11(#[trigger] result@[j], s.len() as int),
                    result@ == seq![Header { name: p.0, start, end }] + tail@.subrange(0, i as int),
                decreases tail.len() - i,
            {
                let h = &tail[i];
                result.push(Header { name: h.name, start: h.start, end: h.end });
                i += 1;
            }
            Some(result)
        },
    }
}

pub open spec fn stage11_request_spec(s: Seq<u8>) -> Option<(bool, int, int, Seq<Header>)> {
    let head = matches(s, 0, b"HEAD "@);
    let start = if head { 5int } else { 4int };
    if s.len() > MAX_REQUEST || !(head || matches(s, 0, b"GET "@)) || start >= s.len() || s[start] != b'/' { None }
    else {
        let end = path_end(s, start);
        if end - start > MAX_PATH { None }
        else if !(matches(s, end, b" HTTP/1.0\r\n"@) || matches(s, end, b" HTTP/1.1\r\n"@)) { None }
        else {
            match header_spec(s, end + 11, MAX_HEADERS as nat) {
                None => None,
                Some(h) => Some((head, start, end, h)),
            }
        }
    }
}

fn parse_stage11(s: &[u8]) -> (r: Option<RawRequest>)
    ensures match r {
        None => stage11_request_spec(s@).is_none(),
        Some(q) => stage11_request_spec(s@) == Some((q.head, q.path_start as int, q.path_end as int, q.headers@))
            && s.len() <= MAX_REQUEST
            && q.path_start < q.path_end <= s.len()
            && q.path_end - q.path_start <= MAX_PATH
            && q.headers.len() <= MAX_HEADERS
            && forall|j: int| 0 <= j < q.headers.len() ==> bounded_stage11(#[trigger] q.headers@[j], s.len() as int),
    },
{
    if s.len() > MAX_REQUEST { return None; }
    let head = equal(s, 0, b"HEAD ");
    let start: usize = if head { 5 } else { 4 };
    if !(head || equal(s, 0, b"GET ")) || start >= s.len() || s[start] != b'/' { return None; }
    let end = scan_path(s, start);
    assert(start < end);
    if end - start > MAX_PATH { return None; }
    if !(equal(s, end, b" HTTP/1.0\r\n") || equal(s, end, b" HTTP/1.1\r\n")) { return None; }
    match headers(s, end + 11, MAX_HEADERS) {
        None => None,
        Some(h) => Some(RawRequest { head, path_start: start, path_end: end, headers: h }),
    }
}
pub open spec fn bounded_raw(h: Header, len: int) -> bool {
    h.name <= 44 && 0 <= h.start < h.end <= len && h.end - h.start <= MAX_VALUE
}

// Declarative header grammar, returning exactly the header fields in wire order.
pub open spec fn raw_header_spec(s: Seq<u8>, at: int, remaining: nat) -> Option<Seq<Header>>
    recommends 0 <= at <= s.len(),
    decreases remaining,
{
    if matches(s, at, b"\r\n"@) && at + 2 == s.len() { Some(Seq::empty()) }
    else if remaining == 0 { None }
    else {
        let p = raw_prefix_spec(s, at);
        let start = at + p.1;
        let end = value_end(s, start);
        if p.1 == 0 || end <= start || end - start > MAX_VALUE
            || s[start] == b' ' || s[end - 1] == b' '
            || !matches(s, end, b"\r\n"@) { None }
        else {
            match raw_header_spec(s, end + 2, (remaining - 1) as nat) {
                None => None,
                Some(tail) => Some(seq![Header { name: p.0, start: start as usize, end: end as usize }] + tail),
            }
        }
    }
}

fn raw_headers(s: &[u8], at: usize, remaining: usize) -> (r: Option<Vec<Header>>)
    requires s.len() <= MAX_REQUEST, at <= s.len(), remaining <= MAX_HEADERS,
    ensures match r {
        None => raw_header_spec(s@, at as int, remaining as nat).is_none(),
        Some(h) => raw_header_spec(s@, at as int, remaining as nat) == Some(h@)
            && h.len() <= remaining
            && forall|j: int| 0 <= j < h.len() ==> bounded_raw(#[trigger] h@[j], s.len() as int),
    },
    decreases remaining,
{
    if equal(s, at, b"\r\n") && at + 2 == s.len() { return Some(Vec::new()); }
    if remaining == 0 { return None; }
    let p = raw_prefix(s, at);
    if p.1 == 0 { return None; }
    let start = at + p.1;
    let end = scan_value(s, start);
    if end <= start || end - start > MAX_VALUE || s[start] == b' ' || s[end - 1] == b' '
        || !equal(s, end, b"\r\n") { return None; }
    match raw_headers(s, end + 2, remaining - 1) {
        None => None,
        Some(tail) => {
            let mut result = Vec::new();
            result.push(Header { name: p.0, start, end });
            let mut i = 0;
            while i < tail.len()
                invariant i <= tail.len(), tail.len() < remaining,
                    forall|j: int| 0 <= j < tail.len() ==> bounded_raw(#[trigger] tail@[j], s.len() as int),
                    forall|j: int| 0 <= j < result.len() ==> bounded_raw(#[trigger] result@[j], s.len() as int),
                    result@ == seq![Header { name: p.0, start, end }] + tail@.subrange(0, i as int),
                decreases tail.len() - i,
            {
                let h = &tail[i];
                result.push(Header { name: h.name, start: h.start, end: h.end });
                i += 1;
            }
            Some(result)
        },
    }
}

pub open spec fn raw_request_spec(s: Seq<u8>) -> Option<(bool, int, int, Seq<Header>)> {
    let head = matches(s, 0, b"HEAD "@);
    let start = if head { 5int } else { 4int };
    if s.len() > MAX_REQUEST || !(head || matches(s, 0, b"GET "@)) || start >= s.len() || s[start] != b'/' { None }
    else {
        let end = path_end(s, start);
        if end - start > MAX_PATH { None }
        else if !(matches(s, end, b" HTTP/1.0\r\n"@) || matches(s, end, b" HTTP/1.1\r\n"@)) { None }
        else {
            match raw_header_spec(s, end + 11, MAX_HEADERS as nat) {
                None => None,
                Some(h) => Some((head, start, end, h)),
            }
        }
    }
}

fn parse_raw(s: &[u8]) -> (r: Option<RawRequest>)
    ensures match r {
        None => raw_request_spec(s@).is_none(),
        Some(q) => raw_request_spec(s@) == Some((q.head, q.path_start as int, q.path_end as int, q.headers@))
            && s.len() <= MAX_REQUEST
            && q.path_start < q.path_end <= s.len()
            && q.path_end - q.path_start <= MAX_PATH
            && q.headers.len() <= MAX_HEADERS
            && forall|j: int| 0 <= j < q.headers.len() ==> bounded_raw(#[trigger] q.headers@[j], s.len() as int),
    },
{
    if s.len() > MAX_REQUEST { return None; }
    let head = equal(s, 0, b"HEAD ");
    let start: usize = if head { 5 } else { 4 };
    if !(head || equal(s, 0, b"GET ")) || start >= s.len() || s[start] != b'/' { return None; }
    let end = scan_path(s, start);
    assert(start < end);
    if end - start > MAX_PATH { return None; }
    if !(equal(s, end, b" HTTP/1.0\r\n") || equal(s, end, b" HTTP/1.1\r\n")) { return None; }
    match raw_headers(s, end + 11, MAX_HEADERS) {
        None => None,
        Some(h) => Some(RawRequest { head, path_start: start, path_end: end, headers: h }),
    }
}
// Classes 0..19 are quiet, 19..28 are hard-reject, 28..44 guarded, 44 unknown.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Policy {
    pub unknown_drop: bool,
    pub guarded_drop: u16,
    pub log_limit: u16,
}
impl Policy {
    pub open spec fn valid(&self) -> bool { self.log_limit <= 600 }
    pub open spec fn strict(&self) -> bool { !self.unknown_drop && self.guarded_drop == 0 }
    pub fn builtin() -> (p: Self) ensures p.valid(), !p.unknown_drop == false,
        p.guarded_drop == 0, p.log_limit == 60,
    { Self { unknown_drop: true, guarded_drop: 0, log_limit: 60 } }
    pub fn is_strict(&self) -> (b: bool) ensures b == self.strict()
    { !self.unknown_drop && self.guarded_drop == 0 }
}
// No Use variant exists. In particular, configuration cannot manufacture one.
#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub head: bool,
    pub path_start: usize,
    pub path_end: usize,
}
pub open spec fn drops(p: &Policy, class: u8) -> bool {
    if class < 19 { true }
    else if class < 28 { false }
    else if class < 44 { (p.guarded_drop >> (class - 28)) & 1 == 1 }
    else { p.unknown_drop }
}
pub fn drop_header(p: &Policy, class: u8) -> (b: bool)
    ensures b == drops(p, class), 19 <= class < 28 ==> !b,
{
    if class < 19 { true }
    else if class < 28 { false }
    else if class < 44 { (p.guarded_drop >> (class - 28)) & 1 == 1 }
    else { p.unknown_drop }
}
pub proof fn hard_reject_is_fixed(p: &Policy, class: u8)
    requires 19 <= class < 28,
    ensures !drops(p, class),
{}
pub open spec fn token(c: u8) -> bool {
    path_char(c) && c != b'/' || c == b'!' || c == b'#' || c == b'$'
    || c == b'%' || c == b'&' || c == 39 || c == b'*' || c == b'+'
    || c == b'^' || c == b'`' || c == b'|' || c == b'~'
}
fn is_token(c: u8) -> (b: bool) ensures b == token(c)
{
    (is_path(c) && c != b'/') || c == b'!' || c == b'#' || c == b'$'
    || c == b'%' || c == b'&' || c == 39 || c == b'*' || c == b'+'
    || c == b'^' || c == b'`' || c == b'|' || c == b'~'
}
pub open spec fn name_end(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(), decreases s.len() - at,
{ if at < s.len() && token(s[at]) { name_end(s, at + 1) } else { at } }
fn scan_name(s: &[u8], at: usize) -> (end: usize)
    requires at <= s.len(),
    ensures end == name_end(s@, at as int), at <= end <= s.len(),
{
    let mut i = at;
    while i < s.len() && is_token(s[i])
        invariant at <= i <= s.len(), name_end(s@, at as int) == name_end(s@, i as int),
        decreases s.len() - i,
    { i += 1; }
    i
}
pub open spec fn class_spec(s: Seq<u8>) -> u8 {
    if s.len() == 4 && matches_header(s, 0, b"Host"@) { 0 }
    else if s.len() == 6 && matches_header(s, 0, b"Accept"@) { 1 }
    else if s.len() == 10 && matches_header(s, 0, b"Connection"@) { 2 }
    else if s.len() == 10 && matches_header(s, 0, b"User-Agent"@) { 3 }
    else if s.len() == 15 && matches_header(s, 0, b"Accept-Language"@) { 4 }
    else if s.len() == 15 && matches_header(s, 0, b"Accept-Encoding"@) { 5 }
    else if s.len() == 25 && matches_header(s, 0, b"Upgrade-Insecure-Requests"@) { 6 }
    else if s.len() == 8 && matches_header(s, 0, b"Priority"@) { 7 }
    else if s.len() == 7 && matches_header(s, 0, b"Referer"@) { 8 }
    else if s.len() == 14 && matches_header(s, 0, b"Sec-Fetch-Dest"@) { 9 }
    else if s.len() == 14 && matches_header(s, 0, b"Sec-Fetch-Site"@) { 10 }
    else if s.len() == 14 && matches_header(s, 0, b"Sec-Fetch-Mode"@) { 11 }
    else if s.len() == 14 && matches_header(s, 0, b"Sec-Fetch-User"@) { 12 }
    else if s.len() == 13 && matches_header(s, 0, b"Cache-Control"@) { 13 }
    else if s.len() == 7 && matches_header(s, 0, b"Sec-GPC"@) { 14 }
    else if s.len() == 9 && matches_header(s, 0, b"Sec-CH-UA"@) { 15 }
    else if s.len() == 16 && matches_header(s, 0, b"Sec-CH-UA-Mobile"@) { 16 }
    else if s.len() == 18 && matches_header(s, 0, b"Sec-CH-UA-Platform"@) { 17 }
    else if s.len() == 6 && matches_header(s, 0, b"Pragma"@) { 18 }
    else if s.len() == 14 && matches_header(s, 0, b"Content-Length"@) { 19 }
    else if s.len() == 17 && matches_header(s, 0, b"Transfer-Encoding"@) { 20 }
    else if s.len() == 2 && matches_header(s, 0, b"TE"@) { 21 }
    else if s.len() == 7 && matches_header(s, 0, b"Trailer"@) { 22 }
    else if s.len() == 6 && matches_header(s, 0, b"Expect"@) { 23 }
    else if s.len() == 7 && matches_header(s, 0, b"Upgrade"@) { 24 }
    else if s.len() == 14 && matches_header(s, 0, b"HTTP2-Settings"@) { 25 }
    else if s.len() == 16 && matches_header(s, 0, b"Proxy-Connection"@) { 26 }
    else if s.len() == 10 && matches_header(s, 0, b"Keep-Alive"@) { 27 }
    else if s.len() == 6 && matches_header(s, 0, b"Cookie"@) { 28 }
    else if s.len() == 13 && matches_header(s, 0, b"Authorization"@) { 29 }
    else if s.len() == 19 && matches_header(s, 0, b"Proxy-Authorization"@) { 30 }
    else if s.len() == 5 && matches_header(s, 0, b"Range"@) { 31 }
    else if s.len() == 8 && matches_header(s, 0, b"If-Range"@) { 32 }
    else if s.len() == 8 && matches_header(s, 0, b"If-Match"@) { 33 }
    else if s.len() == 13 && matches_header(s, 0, b"If-None-Match"@) { 34 }
    else if s.len() == 17 && matches_header(s, 0, b"If-Modified-Since"@) { 35 }
    else if s.len() == 19 && matches_header(s, 0, b"If-Unmodified-Since"@) { 36 }
    else if s.len() == 9 && matches_header(s, 0, b"Forwarded"@) { 37 }
    else if s.len() == 15 && matches_header(s, 0, b"X-Forwarded-For"@) { 38 }
    else if s.len() == 16 && matches_header(s, 0, b"X-Forwarded-Host"@) { 39 }
    else if s.len() == 17 && matches_header(s, 0, b"X-Forwarded-Proto"@) { 40 }
    else if s.len() == 9 && matches_header(s, 0, b"X-Real-IP"@) { 41 }
    else if s.len() == 3 && matches_header(s, 0, b"Via"@) { 42 }
    else if s.len() == 12 && matches_header(s, 0, b"Max-Forwards"@) { 43 }
    else { 44 }
}
pub fn classify(s: &[u8]) -> (id: u8) ensures id == class_spec(s@), id <= 44,
{
    if s.len() == 4 && equal_header(s, 0, b"Host") { 0 }
    else if s.len() == 6 && equal_header(s, 0, b"Accept") { 1 }
    else if s.len() == 10 && equal_header(s, 0, b"Connection") { 2 }
    else if s.len() == 10 && equal_header(s, 0, b"User-Agent") { 3 }
    else if s.len() == 15 && equal_header(s, 0, b"Accept-Language") { 4 }
    else if s.len() == 15 && equal_header(s, 0, b"Accept-Encoding") { 5 }
    else if s.len() == 25 && equal_header(s, 0, b"Upgrade-Insecure-Requests") { 6 }
    else if s.len() == 8 && equal_header(s, 0, b"Priority") { 7 }
    else if s.len() == 7 && equal_header(s, 0, b"Referer") { 8 }
    else if s.len() == 14 && equal_header(s, 0, b"Sec-Fetch-Dest") { 9 }
    else if s.len() == 14 && equal_header(s, 0, b"Sec-Fetch-Site") { 10 }
    else if s.len() == 14 && equal_header(s, 0, b"Sec-Fetch-Mode") { 11 }
    else if s.len() == 14 && equal_header(s, 0, b"Sec-Fetch-User") { 12 }
    else if s.len() == 13 && equal_header(s, 0, b"Cache-Control") { 13 }
    else if s.len() == 7 && equal_header(s, 0, b"Sec-GPC") { 14 }
    else if s.len() == 9 && equal_header(s, 0, b"Sec-CH-UA") { 15 }
    else if s.len() == 16 && equal_header(s, 0, b"Sec-CH-UA-Mobile") { 16 }
    else if s.len() == 18 && equal_header(s, 0, b"Sec-CH-UA-Platform") { 17 }
    else if s.len() == 6 && equal_header(s, 0, b"Pragma") { 18 }
    else if s.len() == 14 && equal_header(s, 0, b"Content-Length") { 19 }
    else if s.len() == 17 && equal_header(s, 0, b"Transfer-Encoding") { 20 }
    else if s.len() == 2 && equal_header(s, 0, b"TE") { 21 }
    else if s.len() == 7 && equal_header(s, 0, b"Trailer") { 22 }
    else if s.len() == 6 && equal_header(s, 0, b"Expect") { 23 }
    else if s.len() == 7 && equal_header(s, 0, b"Upgrade") { 24 }
    else if s.len() == 14 && equal_header(s, 0, b"HTTP2-Settings") { 25 }
    else if s.len() == 16 && equal_header(s, 0, b"Proxy-Connection") { 26 }
    else if s.len() == 10 && equal_header(s, 0, b"Keep-Alive") { 27 }
    else if s.len() == 6 && equal_header(s, 0, b"Cookie") { 28 }
    else if s.len() == 13 && equal_header(s, 0, b"Authorization") { 29 }
    else if s.len() == 19 && equal_header(s, 0, b"Proxy-Authorization") { 30 }
    else if s.len() == 5 && equal_header(s, 0, b"Range") { 31 }
    else if s.len() == 8 && equal_header(s, 0, b"If-Range") { 32 }
    else if s.len() == 8 && equal_header(s, 0, b"If-Match") { 33 }
    else if s.len() == 13 && equal_header(s, 0, b"If-None-Match") { 34 }
    else if s.len() == 17 && equal_header(s, 0, b"If-Modified-Since") { 35 }
    else if s.len() == 19 && equal_header(s, 0, b"If-Unmodified-Since") { 36 }
    else if s.len() == 9 && equal_header(s, 0, b"Forwarded") { 37 }
    else if s.len() == 15 && equal_header(s, 0, b"X-Forwarded-For") { 38 }
    else if s.len() == 16 && equal_header(s, 0, b"X-Forwarded-Host") { 39 }
    else if s.len() == 17 && equal_header(s, 0, b"X-Forwarded-Proto") { 40 }
    else if s.len() == 9 && equal_header(s, 0, b"X-Real-IP") { 41 }
    else if s.len() == 3 && equal_header(s, 0, b"Via") { 42 }
    else if s.len() == 12 && equal_header(s, 0, b"Max-Forwards") { 43 }
    else { 44 }
}

pub open spec fn raw_prefix_spec(s: Seq<u8>, at: int) -> (u8, int) {
    let known = header_prefix(s, at);
    if known.1 > 0 { known }
    else {
        let end = name_end(s, at);
        if end <= at || end - at > 64 || !matches(s, end, b": "@) { (44, 0) }
        else { (class_spec(s.subrange(at, end)), end - at + 2) }
    }
}
fn raw_prefix(s: &[u8], at: usize) -> (r: (u8, usize))
    requires at <= s.len(), s.len() <= MAX_REQUEST,
    ensures r.0 == raw_prefix_spec(s@, at as int).0,
        r.1 as int == raw_prefix_spec(s@, at as int).1,
        r.0 <= 44, r.1 > 0 ==> at + r.1 <= s.len(),
{
    let known = prefix(s, at);
    if known.1 > 0 { return known; }
    let end = scan_name(s, at);
    if end <= at || end - at > 64 || !equal(s, end, b": ") { return (44, 0); }
    (classify(&s[at..end]), end - at + 2)
}
pub open spec fn no_duplicates(h: Seq<Header>) -> bool {
    forall|i: int, j: int| 0 <= i < j < h.len() && (h[i].name == 0 || h[i].name == 2)
        ==> h[i].name != h[j].name
}
pub open spec fn allowed(p: &Policy, h: Seq<Header>) -> bool {
    no_duplicates(h) && forall|i: int| 0 <= i < h.len() ==> drops(p, h[i].name)
}
fn check(p: &Policy, h: &[Header]) -> (ok: bool)
    ensures ok == allowed(p, h@),
{
    let mut host = false;
    let mut connection = false;
    let mut i = 0;
    while i < h.len()
        invariant i <= h.len(),
            host == (exists|j: int| 0 <= j < i && h@[j].name == 0),
            connection == (exists|j: int| 0 <= j < i && h@[j].name == 2),
            forall|j: int| 0 <= j < i ==> drops(p, h@[j].name),
            no_duplicates(h@.subrange(0, i as int)),
        decreases h.len() - i,
    {
        let id = h[i].name;
        if !drop_header(p, id) { return false; }
        if id == 0 { if host { return false; } host = true; }
        if id == 2 { if connection { return false; } connection = true; }
        i += 1;
    }
    assert(h@.subrange(0, i as int) =~= h@);
    true
}
pub open spec fn selected_spec(s: Seq<u8>, p: &Policy) -> Option<(bool, int, int, Seq<Header>)> {
    if p.strict() { stage11_request_spec(s) } else { raw_request_spec(s) }
}
pub open spec fn request_spec(s: Seq<u8>, p: &Policy) -> Option<(bool, int, int)> {
    match selected_spec(s, p) {
        Some((head, start, end, h)) => if allowed(p, h) { Some((head, start, end)) } else { None },
        None => None,
    }
}
pub fn parse_policy(s: &[u8], p: &Policy) -> (r: Option<Request>)
    requires p.valid(),
    ensures match r {
        None => request_spec(s@, p).is_none(),
        Some(q) => request_spec(s@, p) == Some((q.head, q.path_start as int, q.path_end as int))
            && s.len() <= MAX_REQUEST && q.path_start < q.path_end <= s.len()
            && q.path_end - q.path_start <= MAX_PATH,
    },
{
    let raw = if p.is_strict() { parse_stage11(s) } else { parse_raw(s) };
    match raw {
        None => None,
        Some(q) => {
            if !check(p, q.headers.as_slice()) { return None; }
            Some(Request { head: q.head, path_start: q.path_start, path_end: q.path_end })
        },
    }
}
pub fn parse(s: &[u8]) -> (r: Option<Request>)
    ensures match r {
        None => request_spec(s@, &Policy { unknown_drop: true, guarded_drop: 0, log_limit: 60 }).is_none(),
        Some(q) => request_spec(s@, &Policy { unknown_drop: true, guarded_drop: 0, log_limit: 60 })
            == Some((q.head, q.path_start as int, q.path_end as int)),
    },
{ parse_policy(s, &Policy::builtin()) }

// This theorem uses the retained, unchanged stage-11 grammar, not a new oracle.
pub proof fn strict_equivalence(s: Seq<u8>, p: &Policy)
    requires p.strict(),
    ensures request_spec(s, p).is_some() == (stage11_request_spec(s).is_some()
        && no_duplicates(stage11_request_spec(s).unwrap().3)),
        request_spec(s, p).is_some() ==> request_spec(s, p).unwrap()
            == (stage11_request_spec(s).unwrap().0, stage11_request_spec(s).unwrap().1,
                stage11_request_spec(s).unwrap().2),
{
    if let Some((head, start, end, h)) = stage11_request_spec(s) {
        stage11_names(s);
        assert forall|i: int| 0 <= i < h.len() implies drops(p, h[i].name) by {
            assert(h[i].name < 19);
        }
    }
}
// Structural induction supplies the quiet-name fact for the legacy reference.
proof fn stage11_header_names(s: Seq<u8>, at: int, remaining: nat)
    requires header_spec(s, at, remaining).is_some(),
    ensures forall|i: int| 0 <= i < header_spec(s, at, remaining).unwrap().len()
        ==> header_spec(s, at, remaining).unwrap()[i].name < 19,
    decreases remaining,
{
    if !(matches(s, at, b"\r\n"@) && at + 2 == s.len()) {
        let p = header_prefix(s, at);
        let end = value_end(s, at + p.1);
        stage11_header_names(s, end + 2, (remaining - 1) as nat);
    }
}
proof fn stage11_names(s: Seq<u8>)
    requires stage11_request_spec(s).is_some(),
    ensures forall|i: int| 0 <= i < stage11_request_spec(s).unwrap().3.len()
        ==> stage11_request_spec(s).unwrap().3[i].name < 19,
{
    let start = if matches(s, 0, b"HEAD "@) { 5int } else { 4int };
    stage11_header_names(s, path_end(s, start) + 11, MAX_HEADERS as nat);
}

fn whitespace(c: u8) -> bool { c == b' ' || c == b'\t' || c == b'\r' }
fn words(line: &[u8]) -> (out: Vec<&[u8]>)
    requires line.len() <= 4096,
    ensures out.len() <= line.len(),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < line.len()
        invariant i <= line.len() <= 4096, out.len() <= i,
        decreases line.len() - i,
    {
        if line[i] == b'#' { break; }
        if whitespace(line[i]) { i += 1; continue; }
        let start = i;
        while i < line.len() && !whitespace(line[i]) && line[i] != b'#'
            invariant start <= i <= line.len() <= 4096, out.len() <= start,
            decreases line.len() - i,
        { i += 1; }
        if i > start { out.push(&line[start..i]); }
        else { break; }
    }
    out
}
fn action(s: &[u8]) -> (r: Option<bool>) {
    if equal(s, 0, b"drop") && s.len() == 4 { Some(true) }
    else if equal(s, 0, b"reject") && s.len() == 6 { Some(false) }
    else { None }
}
fn number(s: &[u8]) -> (r: Option<u16>) ensures r.is_some() ==> r.unwrap() <= 600,
{
    if s.len() == 0 { return None; }
    let mut n: u16 = 0;
    let mut i = 0;
    while i < s.len()
        invariant i <= s.len(), n <= 600,
        decreases s.len() - i,
    {
        if s[i] < b'0' || s[i] > b'9' { return None; }
        let next = n * 10 + (s[i] - b'0') as u16;
        if next > 600 { return None; }
        n = next;
        i += 1;
    }
    Some(n)
}
pub fn parse_config(s: &[u8]) -> (r: Option<Policy>)
    ensures r.is_some() ==> r.unwrap().valid(),
{
    if s.len() > 4096 { return None; }
    let mut i = 0;
    while i < s.len()
        invariant i <= s.len() <= 4096,
        decreases s.len() - i,
    {
        if s[i] > 127 || (s[i] < 32 && s[i] != 9 && s[i] != 10 && s[i] != 13) { return None; }
        i += 1;
    }
    let mut policy = Policy::builtin();
    let mut seen_guarded: u16 = 0;
    let mut seen_unknown = false;
    let mut seen_limit = false;
    let mut at = 0;
    let mut lines = 0;
    while at < s.len()
        invariant at <= s.len() <= 4096, lines <= 64, policy.valid(),
        decreases s.len() - at,
    {
        if lines == 64 { return None; }
        lines += 1;
        let mut end = at;
        while end < s.len() && s[end] != b'\n'
            invariant at <= end <= s.len() <= 4096,
            decreases s.len() - end,
        { end += 1; }
        let tokens = words(&s[at..end]);
        if tokens.len() > 0 {
            if tokens.len() == 2 && equal(tokens[0], 0, b"unknown") && tokens[0].len() == 7 {
                if seen_unknown { return None; }
                seen_unknown = true;
                match action(tokens[1]) { Some(b) => policy.unknown_drop = b, None => return None }
            } else if tokens.len() == 3 && equal(tokens[0], 0, b"guarded") && tokens[0].len() == 7 {
                let id = classify(tokens[1]);
                if id < 28 || id >= 44 { return None; }
                let bit = 1u16 << (id - 28);
                if seen_guarded & bit != 0 { return None; }
                seen_guarded = seen_guarded | bit;
                match action(tokens[2]) {
                    Some(true) => policy.guarded_drop = policy.guarded_drop | bit,
                    Some(false) => {},
                    None => return None,
                }
            } else if tokens.len() == 2 && equal(tokens[0], 0, b"log-unknown-per-minute") && tokens[0].len() == 22 {
                if seen_limit { return None; }
                seen_limit = true;
                match number(tokens[1]) { Some(n) => policy.log_limit = n, None => return None }
            } else { return None; }
        }
        at = if end < s.len() { end + 1 } else { end };
    }
    Some(policy)
}
// This holds for every representable Policy, stronger than only parser outputs.
pub proof fn config_cannot_loosen(p: &Policy)
    ensures forall|class: u8| 19 <= class < 28 ==> !drops(p, class),
{}

// The accepted header list is private; every class obeys policy and no hard
// class occurs. No request field contains a header range or header value.
pub proof fn accepted_safety(s: Seq<u8>, p: &Policy)
    requires request_spec(s, p).is_some(),
    ensures selected_spec(s, p).is_some(), no_duplicates(selected_spec(s, p).unwrap().3),
        forall|i: int| 0 <= i < selected_spec(s, p).unwrap().3.len() ==>
            drops(p, selected_spec(s, p).unwrap().3[i].name)
            && !(19 <= selected_spec(s, p).unwrap().3[i].name < 28),
{}
}

// Stage 13: ghost-only checks of specification intent.
mod intent;
