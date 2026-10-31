use super::*;

verus! {
pub open spec fn nonce_spec(iv: Seq<u8>, seq: u64) -> Seq<u8> {
    Seq::new(12, |i: int| iv[i] ^ (if i < 4 { 0u8 }
        else { ((seq >> ((11 - i) * 8) as u64) & 255) as u8 }))
}
}
