use super::*;

verus! {
pub open spec fn all_zero(s: Seq<u8>) -> bool {
    forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i] == 0
}
}
