//! Bounded, total first-record ClientHello inspection. No record reassembly/HRR.
#[path = "spec/hello.rs"]
pub mod spec;
pub use self::spec::*;
use vstd::prelude::*;
#[path = "hello_intent.rs"]
mod intent;

verus! {
pub const RECORD_LIMIT: usize = 16645;
pub const GROUP_LIMIT: usize = 128;
#[derive(Debug, PartialEq, Eq)]
pub struct Choice {
    pub suite: u16,
    pub group: u16,
    pub signature: u16,
    pub offered_groups: Vec<u16>,
    pub sni: Vec<u8>,
}

fn word(s: &[u8], p: usize) -> (v: u16)
    requires p + 2 <= s.len(),
    ensures v == be16(s@, p as int),
{ (s[p] as u16) * 256 + s[p + 1] as u16 }

// An absent/malformed vector has no end. All arithmetic is bounded by RECORD_LIMIT.
fn vector(s: &[u8], p: usize) -> (v: Option<usize>)
    requires s.len() <= RECORD_LIMIT, p <= s.len(),
    ensures v == vector_spec(s@, p as int),
        v is Some ==> p + 2 <= v->Some_0 <= s.len(),
{
    if s.len() - p < 2 { return None; }
    let n = word(s, p) as usize;
    if n > s.len() - p - 2 { None } else { Some(p + 2 + n) }
}

fn layout(s: &[u8]) -> (v: Option<(usize, usize, usize)>)
    ensures v == layout_spec(s@),
        v is Some ==> s.len() <= RECORD_LIMIT
            && v->Some_0.0 <= v->Some_0.1 <= v->Some_0.2 <= s.len(),
{
    if s.len() < 44 || s.len() > RECORD_LIMIT { return None; }
    if s[0] != 22 || s[1] != 3 || s[2] < 1 || s[2] > 3
        || word(s, 3) as usize != s.len() - 5 || s[5] != 1
        || s[6] != 0 || word(s, 7) as usize != s.len() - 9
        || s[9] != 3 || s[10] != 3 { return None; }
    let sid = s[43] as usize;
    if sid > 32 || sid > s.len() - 44 { return None; }
    let p = 44 + sid;
    let end = match vector(s, p) { Some(v) => v, None => return None };
    if end - p <= 2 || (end - p - 2) % 2 != 0 { return None; }
    if s.len() - end < 2 || s[end] != 1 || s[end + 1] != 0 { return None; }
    let ext = end + 2;
    let last = match vector(s, ext) { Some(v) => v, None => return None };
    if last != s.len() { return None; }
    Some((p + 2, end, ext + 2))
}

// Validate the complete extension framing, and reject duplicate extension types.
fn extensions(s: &[u8], p: usize, seen: &mut Vec<u16>) -> (ok: bool)
    requires s.len() <= RECORD_LIMIT, p <= s.len(), seen.len() <= GROUP_LIMIT,
    ensures ok == extensions_spec(s@, p as int, old(seen)@),
    decreases s.len() - p,
{
    if p == s.len() { return true; }
    if s.len() - p < 4 || seen.len() == GROUP_LIMIT { return false; }
    let kind = word(s, p);
    if contains(seen, kind) { return false; }
    let end = match vector(s, p + 2) { Some(v) => v, None => return false };
    seen.push(kind);
    extensions(s, end, seen)
}

fn contains(s: &Vec<u16>, x: u16) -> (yes: bool)
    ensures yes == member_spec(s@, x),
{ proof { assert(s@.subrange(0, s.len() as int) =~= s@); } contains_at(s, 0, x) }

fn contains_at(s: &Vec<u16>, p: usize, x: u16) -> (yes: bool)
    requires p <= s.len(),
    ensures yes == member_spec(s@.subrange(p as int, s.len() as int), x),
    decreases s.len() - p,
{
    if p == s.len() { return false; }
    proof { assert(s@.subrange(p as int, s.len() as int).subrange(1, (s.len() - p) as int)
        =~= s@.subrange(p + 1, s.len() as int)); }
    s[p] == x || contains_at(s, p + 1, x)
}

fn find(s: &[u8], p: usize, kind: u16, fuel: usize) -> (v: Option<(usize, usize)>)
    requires s.len() <= RECORD_LIMIT, p <= s.len(),
    ensures v == find_spec(s@, p as int, kind, fuel as int),
        v is Some ==> v->Some_0.0 <= v->Some_0.1 <= s.len(),
    decreases s.len() - p,
{
    if fuel == 0 || s.len() - p < 4 { return None; }
    let end = match vector(s, p + 2) { Some(v) => v, None => return None };
    if word(s, p) == kind { Some((p + 4, end)) } else { find(s, end, kind, fuel - 1) }
}

fn words(s: &[u8], p: usize, end: usize) -> (v: Option<Vec<u16>>)
    requires s.len() <= RECORD_LIMIT, p <= end <= s.len(),
    ensures match v {
        Some(v) => words_spec(s@, p as int, end as int) == Some(v@),
        None => words_spec(s@, p as int, end as int) is None,
    },
    decreases end - p,
{
    if end - p > 256 { return None; }
    if p == end { return Some(Vec::new()); }
    if end - p < 2 { return None; }
    let x = word(s, p);
    let tail = match words(s, p + 2, end) { Some(v) => v, None => return None };
    let mut out = Vec::new();
    out.push(x);
    let mut i = 0;
    while i < tail.len()
        invariant i <= tail.len(), out@ == seq![x] + tail@.subrange(0, i as int),
        decreases tail.len() - i,
    {
        out.push(tail[i]);
        i += 1;
    }
    Some(out)
}

fn list(s: &[u8], ext: usize, kind: u16) -> (v: Option<Vec<u16>>)
    requires s.len() <= RECORD_LIMIT, ext <= s.len(),
    ensures match v {
        Some(v) => list_spec(s@, ext as int, kind) == Some(v@),
        None => list_spec(s@, ext as int, kind) is None,
    },
{
    let (p, end) = match find(s, ext, kind, GROUP_LIMIT) { Some(v) => v, None => return None };
    let last = match vector(s, p) { Some(v) => v, None => return None };
    if last != end { return None; }
    words(s, p + 2, end)
}

fn shares(s: &[u8], p: usize, end: usize, group: u16, fuel: usize) -> (v: Option<bool>)
    requires s.len() <= RECORD_LIMIT, p <= end <= s.len(),
    ensures v == shares_spec(s@, p as int, end as int, group, fuel as int),
    decreases end - p,
{
    if p == end { return Some(false); }
    if fuel == 0 || end - p < 4 { return None; }
    let key_end = match vector(s, p + 2) { Some(v) => v, None => return None };
    if key_end > end || key_end == p + 4 { return None; }
    let rest = match shares(s, key_end, end, group, fuel - 1) { Some(v) => v, None => return None };
    Some(word(s, p) == group || rest)
}

fn has_share(s: &[u8], ext: usize, group: u16) -> (v: bool)
    requires s.len() <= RECORD_LIMIT, ext <= s.len(),
    ensures v == share_spec(s@, ext as int, group),
{
    let (p, end) = match find(s, ext, 51, GROUP_LIMIT) { Some(v) => v, None => return false };
    let last = match vector(s, p) { Some(v) => v, None => return false };
    if last != end { return false; }
    shares(s, p + 2, end, group, GROUP_LIMIT) == Some(true)
}

pub fn choose(s: &[u8]) -> (v: Option<Choice>)
    ensures match v {
        Some(c) => choice_spec(s@, c.group) && c.suite == 0x1303 && c.signature == 0x0403
            && c.offered_groups@ == offered_spec(s@) && c.offered_groups.len() <= GROUP_LIMIT
            && c.sni@ == sni_spec(s@),
        None => !(exists|g: u16| choice_spec(s@, g)),
    },
{
    let (p, end, ext) = match layout(s) { Some(v) => v, None => return None };
    let mut seen = Vec::new();
    if !extensions(s, ext, &mut seen) { return None; }
    let suites = match words(s, p, end) { Some(v) => v, None => return None };
    let groups = match list(s, ext, 10) { Some(v) => v, None => return None };
    let sigs = match list(s, ext, 13) { Some(v) => v, None => return None };
    if groups.len() > GROUP_LIMIT || !contains(&suites, 0x1303) || !contains(&sigs, 0x0403) { return None; }
    let hybrid = contains(&groups, 0x11ec) && has_share(s, ext, 0x11ec);
    let x = contains(&groups, 29) && has_share(s, ext, 29);
    let p256 = contains(&groups, 23) && has_share(s, ext, 23);
    let group = if hybrid { 0x11ecu16 } else if x { 29u16 } else if p256 { 23u16 } else { return None; };
    let sni = match server_name(s, ext) { Some(v) => v, None => return None };
    Some(Choice { suite: 0x1303, group, signature: 0x0403, offered_groups: groups, sni })
}

fn server_name(s: &[u8], ext: usize) -> (v: Option<Vec<u8>>)
    requires s.len() <= RECORD_LIMIT, ext <= s.len(),
    ensures match v { Some(v) => name_spec(s@, ext as int) == Some(v@), None => name_spec(s@, ext as int) is None },
{
    let (p, end) = match find(s, ext, 0, GROUP_LIMIT) { Some(v) => v, None => return Some(Vec::new()) };
    // One host_name only; reject other forms rather than inventing routing semantics.
    if end - p < 5 || word(s, p) as usize != end - p - 2 || s[p + 2] != 0
        || word(s, p + 3) as usize != end - p - 5 || end - p > 260 { return None; }
    let mut out = Vec::new();
    let mut i = p + 5;
    while i < end
        invariant p + 5 <= i <= end <= s.len(), out@ == s@.subrange(p + 5, i as int),
        decreases end - i,
    {
        out.push(s[i]);
        i += 1;
    }
    Some(out)
}
}

#[cfg(test)]
mod tests {
    use super::*;
    fn vector(bytes: &[u8]) -> Vec<u8> {
        let mut out = (bytes.len() as u16).to_be_bytes().to_vec();
        out.extend_from_slice(bytes);
        out
    }
    fn fixture(groups: &[u16], shares: &[u16], suite: u16, signature: u16) -> Vec<u8> {
        let words = |values: &[u16]| values.iter().flat_map(|x| x.to_be_bytes()).collect::<Vec<_>>();
        let mut keys = Vec::new();
        for g in shares { keys.extend_from_slice(&g.to_be_bytes()); keys.extend(vector(&[7; 32])); }
        let mut ext = Vec::new();
        for (kind, data) in [(10u16, vector(&words(groups))), (13, vector(&signature.to_be_bytes())), (51, vector(&keys))] {
            ext.extend(kind.to_be_bytes()); ext.extend(vector(&data));
        }
        let mut body = vec![3, 3]; body.extend([0; 32]); body.push(0);
        body.extend(vector(&suite.to_be_bytes())); body.extend([1, 0]); body.extend(vector(&ext));
        let mut hs = vec![1, 0]; hs.extend((body.len() as u16).to_be_bytes()); hs.extend(body);
        let mut record = vec![22, 3, 1]; record.extend(vector(&hs)); record
    }
    #[test]
    fn choices_and_rejections() {
        for (groups, shares, expected) in [
            (vec![29, 23], vec![29], Some(29)),
            (vec![0x11ec, 29, 23], vec![29, 0x11ec], Some(0x11ec)),
            (vec![23, 29], vec![23], Some(23)),
            (vec![0x11ec, 29, 23], vec![29, 23], Some(29)),
            (vec![29], vec![24], None), (vec![24], vec![24], None), (vec![24], vec![29], None),
        ] {
            assert_eq!(choose(&fixture(&groups, &shares, 0x1303, 0x403)).map(|c| c.group), expected);
        }
        assert!(choose(&fixture(&[29], &[29], 0x1301, 0x403)).is_none());
        assert!(choose(&fixture(&[29], &[29], 0x1303, 0x804)).is_none());
        let good = fixture(&[29], &[29], 0x1303, 0x403);
        for n in 0..good.len() { assert!(choose(&good[..n]).is_none()); }
        for n in 0..512 { assert!(choose(&vec![0; n]).is_none()); }
    }
    #[test]
    fn captured_clienthellos() {
        for b in [include_bytes!("../../../tests/hellos/curl-system.bin").as_slice(),
            include_bytes!("../../../tests/hellos/curl-homebrew-openssl.bin").as_slice(),
            include_bytes!("../../../tests/hellos/openssl-s_client.bin").as_slice(),
            include_bytes!("../../../tests/hellos/rustls-client.bin").as_slice()] {
            assert!(choose(b).is_some());
        }
    }
}
