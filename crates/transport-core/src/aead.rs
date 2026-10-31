#[path = "spec/aead.rs"]
pub mod spec;
pub use self::spec::*;
use vstd::prelude::*;
verus! {
pub const KEY_LEN: usize = 32;
pub const IV_LEN: usize = 12;
pub const TAG_LEN: usize = 16;

pub fn nonce(iv: &[u8], seq: u64) -> (out: Option<[u8; 12]>)
    ensures match out {
        Some(n) => iv.len() == IV_LEN && n@ == nonce_spec(iv@, seq),
        None => iv.len() != IV_LEN,
    },
{
    if iv.len() != IV_LEN { return None; }
    let mut out = [0u8; 12];
    let mut i: usize = 0;
    while i < 12
        invariant 0 <= i <= 12, iv.len() == IV_LEN,
            forall|j: int| 0 <= j < i ==> #[trigger] out@[j] == nonce_spec(iv@, seq)[j],
        decreases 12 - i,
    {
        let mask = if i < 4 { 0u8 } else { ((seq >> ((11-i)*8)) & 255) as u8 };
        out[i] = iv[i] ^ mask;
        i += 1;
    }
    assert(out@ =~= nonce_spec(iv@, seq));
    Some(out)
}
pub fn sealed_len(payload: usize) -> (out: Option<usize>)
    ensures match out {
        Some(n) => payload <= usize::MAX - 17 && n == payload + 1 + TAG_LEN,
        None => payload > usize::MAX - 17,
    },
{
    if payload > usize::MAX - 17 { None } else { Some(payload + 17) }
}
pub fn plaintext_len(ciphertext: usize) -> (out: Option<usize>)
    ensures match out {
        Some(n) => ciphertext >= TAG_LEN + 1 && n == ciphertext - TAG_LEN && 1 <= n <= ciphertext,
        None => ciphertext < TAG_LEN + 1,
    },
{
    if ciphertext < TAG_LEN + 1 { None } else { Some(ciphertext - TAG_LEN) }
}
// Crypto itself is an opaque caller operation. This gate proves that a reported
// failure never releases plaintext, and successful copies preserve every byte.
pub fn release(authenticated: bool, plain: &[u8], dest: &mut [u8]) -> (ok: bool)
    requires plain.len() == old(dest).len(),
    ensures ok == authenticated,
        !ok ==> final(dest)@ == old(dest)@,
        ok ==> final(dest)@ == plain@,
{
    if !authenticated { return false; }
    let mut i = 0;
    while i < plain.len()
        invariant i <= plain.len(), dest.len() == plain.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] dest@[j] == plain@[j],
        decreases plain.len() - i,
    { dest[i] = plain[i]; i += 1; }
    assert(dest@ =~= plain@);
    true
}
}
