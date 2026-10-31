use super::*;

verus! {
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

// Applies to every read in an arbitrary connection: each scheduled expiration
// lies within the same fixed budget. Event delivery by the OS is not modeled.
pub proof fn deadline_bound(elapsed: u64, first: Option<u64>, d: u64)
    requires timeout_spec(elapsed, first) == Some(d),
    ensures elapsed as int + d <= match first { None => IDLE_NS as int,
        Some(t) => t as int + REQUEST_NS },
        d > 0,
{}

pub open spec fn admission_result(active: usize) -> Option<usize> {
    if active < MAX_CONNECTIONS { Some((active + 1) as usize) } else { None }
}

pub open spec fn release_result(active: usize) -> usize {
    if active == 0 { 0 } else { (active - 1) as usize }
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

pub(super) proof fn decimal_prefix(s: Seq<u8>, a: int, i: int)
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

pub proof fn sizes_agree()
    ensures crate::aead::KEY_LEN == HASH_LEN, X25519_LEN == 32, P256_POINT_LEN == 65,
{}
}
