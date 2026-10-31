#[path = "spec/record.rs"]
pub mod spec;
pub use self::spec::*;
use vstd::prelude::*;
use crate::aead::{sealed_len, TAG_LEN};
use crate::io::{X25519_LEN, P256_POINT_LEN};

verus! {
#[derive(Debug, PartialEq, Eq)]
pub struct Unpadded { pub typ: u8, pub len: usize }
#[derive(Debug, PartialEq, Eq)]
pub enum UnpadError { Oversized, NoContentType, ChangeCipherSpec }

pub fn unpad(plain: &[u8]) -> (out: Result<Unpadded, UnpadError>)
    ensures
        (out == Err(UnpadError::NoContentType)) <==> (plain.len() <= 16_385 && all_zero(plain@)),
        plain.len() > 16_385 ==> out == Err(UnpadError::Oversized),
        match out {
            Ok(p) => p.len < plain.len() && plain@[p.len as int] == p.typ
                && p.typ != 0 && p.typ != 20 && p.len <= 16_384
                && forall|j: int| p.len < j < plain.len() ==> #[trigger] plain@[j] == 0,
            Err(UnpadError::ChangeCipherSpec) => plain.len() <= 16_385
                && exists|i: int| 0 <= i < plain.len() && plain@[i] == 20
                    && forall|j: int| i < j < plain.len() ==> #[trigger] plain@[j] == 0,
            Err(UnpadError::Oversized) => plain.len() > 16_385,
            _ => true,
        },
{
    if plain.len() > 16_385 { return Err(UnpadError::Oversized); }
    let mut end = plain.len();
    while end > 0
        invariant end <= plain.len() <= 16_385,
            forall|j: int| end <= j < plain.len() ==> #[trigger] plain@[j] == 0,
        decreases end,
    {
        end -= 1;
        let typ = plain[end];
        if typ != 0 {
            if typ == 20 { return Err(UnpadError::ChangeCipherSpec); }
            if end > 16_384 { return Err(UnpadError::Oversized); }
            return Ok(Unpadded { typ, len: end });
        }
    }
    Err(UnpadError::NoContentType)
}

pub fn aad(len: usize) -> (out: [u8; 5])
    ensures out[0] == 0x17, out[1] == 3, out[2] == 3,
        out[3] == ((len >> 8) as u8), out[4] == len as u8,
        len < 65_536 ==> out[3] as int * 256 + out[4] as int == len,
{
    let hi = (len >> 8) as u8;
    let lo = len as u8;
    assert(len < 65_536 ==> ((len >> 8) as u8) as usize * 256 + (len as u8) as usize == len) by (bit_vector);
    [0x17, 3, 3, hi, lo]
}

pub fn inner_plaintext(payload: &[u8], typ: u8) -> (out: Option<Vec<u8>>)
    ensures match out {
        Some(v) => payload.len() <= usize::MAX - 17
            && v@ == payload@ + seq![typ] && v.len() + TAG_LEN == payload.len() + 17,
        None => payload.len() > usize::MAX - 17,
    },
{
    let _len = match sealed_len(payload.len()) { Some(n) => n, None => return None };
    let mut out = Vec::with_capacity(payload.len() + 1);
    let mut i = 0;
    while i < payload.len()
        invariant i <= payload.len(), payload.len() <= usize::MAX - 17,
            out@ == payload@.subrange(0, i as int),
        decreases payload.len() - i,
    { out.push(payload[i]); i += 1; }
    out.push(typ);
    Some(out)
}
pub fn encrypted_payload_len(len: usize) -> (out: usize)
    ensures len <= usize::MAX - 17 ==> out == len + 17,
        len > usize::MAX - 17 ==> out == usize::MAX,
{
    match sealed_len(len) { Some(n) => n, None => usize::MAX }
}
pub fn x25519_peer(peer: &[u8]) -> (out: Option<[u8; X25519_LEN]>)
    ensures match out { Some(p) => peer.len() == X25519_LEN && p@ == peer@, None => peer.len() != X25519_LEN },
{
    if peer.len() != X25519_LEN { return None; }
    let mut out = [0u8; X25519_LEN];
    let mut i = 0;
    while i < X25519_LEN
        invariant i <= X25519_LEN, peer.len() == X25519_LEN,
            forall|j: int| 0 <= j < i ==> #[trigger] out@[j] == peer@[j],
        decreases X25519_LEN - i,
    { out[i] = peer[i]; i += 1; }
    assert(out@ =~= peer@);
    Some(out)
}
pub fn p256_point_shape(public: &[u8]) -> (out: bool)
    ensures out == (public.len() == P256_POINT_LEN && public@[0] == 4),
{ public.len() == P256_POINT_LEN && public[0] == 4 }
pub fn offers(offered: &[u16], wanted: u16) -> (out: bool)
    ensures out == (exists|i: int| 0 <= i < offered.len() && #[trigger] offered@[i] == wanted),
{
    let mut i = 0;
    while i < offered.len()
        invariant i <= offered.len(), forall|j: int| 0 <= j < i ==> #[trigger] offered@[j] != wanted,
        decreases offered.len() - i,
    {
        if offered[i] == wanted { return true; }
        i += 1;
    }
    false
}
}
