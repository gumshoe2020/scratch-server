use super::*;

verus! {
pub open spec fn matches(s: Seq<u8>, at: int, text: Seq<u8>) -> bool {
    0 <= at && at + text.len() <= s.len()
        && s.subrange(at, at + text.len()) == text
}

// ASCII folding applies only to whitelisted header prefixes, never request syntax.
pub open spec fn ascii_lower(c: u8) -> u8 {
    if b'A' <= c <= b'Z' { (c + 32) as u8 } else { c }
}

pub open spec fn matches_header(s: Seq<u8>, at: int, text: Seq<u8>) -> bool {
    0 <= at && at + text.len() <= s.len()
        && forall|j: int| 0 <= j < text.len() ==>
            ascii_lower(s[at + j]) == ascii_lower(text[j])
}

pub open spec fn path_char(c: u8) -> bool {
    (b'a' <= c <= b'z') || (b'A' <= c <= b'Z') || (b'0' <= c <= b'9')
        || c == b'/' || c == b'-' || c == b'_' || c == b'.'
}

// First byte not in the path alphabet, or the end of the input.
pub open spec fn path_end(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    if at < s.len() && path_char(s[at]) { path_end(s, at + 1) } else { at }
}

pub open spec fn value_char(c: u8) -> bool { b' ' <= c <= b'~' }

pub open spec fn value_end(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(),
    decreases s.len() - at,
{
    if at < s.len() && value_char(s[at]) { value_end(s, at + 1) } else { at }
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

impl Policy {
    pub open spec fn valid(&self) -> bool { self.log_limit <= 600 }
}

impl Policy {
    pub open spec fn strict(&self) -> bool { !self.unknown_drop && self.guarded_drop == 0 }
}

pub open spec fn drops(p: &Policy, class: u8) -> bool {
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

pub open spec fn name_end(s: Seq<u8>, at: int) -> int
    recommends 0 <= at <= s.len(), decreases s.len() - at,
{ if at < s.len() && token(s[at]) { name_end(s, at + 1) } else { at } }

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

pub open spec fn raw_prefix_spec(s: Seq<u8>, at: int) -> (u8, int) {
    let known = header_prefix(s, at);
    if known.1 > 0 { known }
    else {
        let end = name_end(s, at);
        if end <= at || end - at > 64 || !matches(s, end, b": "@) { (44, 0) }
        else { (class_spec(s.subrange(at, end)), end - at + 2) }
    }
}

pub open spec fn no_duplicates(h: Seq<Header>) -> bool {
    forall|i: int, j: int| 0 <= i < j < h.len() && (h[i].name == 0 || h[i].name == 2)
        ==> h[i].name != h[j].name
}

pub open spec fn allowed(p: &Policy, h: Seq<Header>) -> bool {
    no_duplicates(h) && forall|i: int| 0 <= i < h.len() ==> drops(p, h[i].name)
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
pub(super) proof fn stage11_header_names(s: Seq<u8>, at: int, remaining: nat)
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

pub(super) proof fn stage11_names(s: Seq<u8>)
    requires stage11_request_spec(s).is_some(),
    ensures forall|i: int| 0 <= i < stage11_request_spec(s).unwrap().3.len()
        ==> stage11_request_spec(s).unwrap().3[i].name < 19,
{
    let start = if matches(s, 0, b"HEAD "@) { 5int } else { 4int };
    stage11_header_names(s, path_end(s, start) + 11, MAX_HEADERS as nat);
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
