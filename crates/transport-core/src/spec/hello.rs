use super::*;
verus! {
pub open spec fn member_spec(s: Seq<u16>, x: u16) -> bool
    decreases s.len(),
{
    s.len() > 0 && (s[0] == x || member_spec(s.subrange(1, s.len() as int), x))
}
pub open spec fn be16(s: Seq<u8>, p: int) -> int { s[p] as int * 256 + s[p + 1] as int }
pub open spec fn vector_spec(s: Seq<u8>, p: int) -> Option<usize> {
    if s.len() > RECORD_LIMIT || p < 0 || p + 2 > s.len() || be16(s, p) > s.len() - p - 2 { None }
    else { Some((p + 2 + be16(s, p)) as usize) }
}
pub open spec fn layout_spec(s: Seq<u8>) -> Option<(usize, usize, usize)> {
    if s.len() < 44 || s.len() > RECORD_LIMIT { None }
    else if s[0] != 22 || s[1] != 3 || s[2] < 1 || s[2] > 3
        || be16(s, 3) != s.len() - 5 || s[5] != 1
        || s[6] != 0 || be16(s, 7) != s.len() - 9 || s[9] != 3 || s[10] != 3 { None }
    else if s[43] > 32 || s[43] > s.len() - 44 { None }
    else {
        let p = 44 + s[43] as int;
        match vector_spec(s, p) {
            None => None,
            Some(end) => if end - p <= 2 || (end - p - 2) % 2 != 0
                || s.len() - end < 2 || s[end as int] != 1 || s[end + 1] != 0 { None }
            else { match vector_spec(s, end + 2) {
                Some(last) => if last == s.len() { Some(((p + 2) as usize, end, (end + 4) as usize)) } else { None },
                None => None,
            } },
        }
    }
}
pub open spec fn extensions_spec(s: Seq<u8>, p: int, seen: Seq<u16>) -> bool
    decreases s.len() - p,
{
    if p == s.len() { true }
    else if p < 0 || s.len() - p < 4 || seen.len() >= GROUP_LIMIT { false }
    else if member_spec(seen, be16(s, p) as u16) { false }
    else { match vector_spec(s, p + 2) {
        None => false,
        Some(end) => extensions_spec(s, end as int, seen.push(be16(s, p) as u16)),
    } }
}
pub open spec fn find_spec(s: Seq<u8>, p: int, kind: u16, fuel: int) -> Option<(usize, usize)>
    decreases s.len() - p,
{
    if fuel <= 0 || p < 0 || s.len() - p < 4 { None }
    else { match vector_spec(s, p + 2) {
        None => None,
        Some(end) => if be16(s, p) == kind { Some(((p + 4) as usize, end)) }
            else { find_spec(s, end as int, kind, fuel - 1) },
    } }
}
pub open spec fn words_spec(s: Seq<u8>, p: int, end: int) -> Option<Seq<u16>>
    decreases end - p,
{
    if end - p > 256 { None }
    else if p == end { Some(Seq::empty()) }
    else if p < 0 || end > s.len() || end - p < 2 { None }
    else { match words_spec(s, p + 2, end) {
        None => None,
        Some(tail) => Some(seq![be16(s, p) as u16] + tail),
    } }
}
pub open spec fn list_spec(s: Seq<u8>, ext: int, kind: u16) -> Option<Seq<u16>> {
    match find_spec(s, ext, kind, GROUP_LIMIT as int) {
        None => None,
        Some((p, end)) => if vector_spec(s, p as int) != Some(end) { None }
            else { words_spec(s, p + 2, end as int) },
    }
}
pub open spec fn shares_spec(s: Seq<u8>, p: int, end: int, group: u16, fuel: int) -> Option<bool>
    decreases end - p,
{
    if p == end { Some(false) }
    else if fuel <= 0 || p < 0 || end > s.len() || end - p < 4 { None }
    else { match vector_spec(s, p + 2) {
        None => None,
        Some(last) => if last > end || last == p + 4 { None }
        else { match shares_spec(s, last as int, end, group, fuel - 1) {
            None => None, Some(rest) => Some(be16(s, p) == group || rest),
        } },
    } }
}
pub open spec fn share_spec(s: Seq<u8>, ext: int, group: u16) -> bool {
    match find_spec(s, ext, 51, GROUP_LIMIT as int) {
        None => false,
        Some((p, end)) => vector_spec(s, p as int) == Some(end)
            && shares_spec(s, p + 2, end as int, group, GROUP_LIMIT as int) == Some(true),
    }
}
pub open spec fn name_spec(s: Seq<u8>, ext: int) -> Option<Seq<u8>> {
    match find_spec(s, ext, 0, GROUP_LIMIT as int) {
        None => Some(Seq::empty()),
        Some((p, end)) => if end - p < 5 || be16(s, p as int) != end - p - 2
            || s[p + 2] != 0 || be16(s, p + 3) != end - p - 5 || end - p > 260 { None }
            else { Some(s.subrange(p + 5, end as int)) },
    }
}
pub open spec fn eligible(s: Seq<u8>, ext: int, groups: Seq<u16>, g: u16) -> bool {
    member_spec(groups, g) && share_spec(s, ext, g)
}
pub open spec fn choice_spec(s: Seq<u8>, g: u16) -> bool {
    match layout_spec(s) {
        None => false,
        Some((p, end, ext)) => extensions_spec(s, ext as int, Seq::empty())
            && words_spec(s, p as int, end as int) is Some
            && member_spec(words_spec(s, p as int, end as int)->Some_0, 0x1303)
            && list_spec(s, ext as int, 10) is Some
            && list_spec(s, ext as int, 13) is Some
            && member_spec(list_spec(s, ext as int, 13)->Some_0, 0x0403)
            && name_spec(s, ext as int) is Some
            && { let groups = list_spec(s, ext as int, 10)->Some_0;
                groups.len() <= GROUP_LIMIT && eligible(s, ext as int, groups, g)
                && (g == 0x11ec || (g == 29 && !eligible(s, ext as int, groups, 0x11ec))
                    || (g == 23 && !eligible(s, ext as int, groups, 0x11ec)
                        && !eligible(s, ext as int, groups, 29))) },
    }
}
pub open spec fn offered_spec(s: Seq<u8>) -> Seq<u16> {
    list_spec(s, layout_spec(s)->Some_0.2 as int, 10)->Some_0
}
pub open spec fn sni_spec(s: Seq<u8>) -> Seq<u8> {
    name_spec(s, layout_spec(s)->Some_0.2 as int)->Some_0
}
}
