//! Concrete structural witnesses; key bytes are synthetic, not cryptographic test vectors.
use super::*;
verus! {
pub open spec fn safari_shaped() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,107u8,1u8,0u8,0u8,103u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,3u8,1u8,0u8,0u8,60u8,0u8,10u8,0u8,6u8,0u8,4u8,0u8,29u8,0u8,23u8,0u8,13u8,0u8,4u8,0u8,2u8,4u8,3u8,0u8,51u8,0u8,38u8,0u8,36u8,0u8,29u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_safari_shaped()
    ensures choice_spec(safari_shaped(), 29),
{
    assert(choice_spec(safari_shaped(), 29)) by (compute)
}
pub open spec fn curl_shaped() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,145u8,1u8,0u8,0u8,141u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,3u8,1u8,0u8,0u8,98u8,0u8,10u8,0u8,8u8,0u8,6u8,17u8,236u8,0u8,29u8,0u8,23u8,0u8,13u8,0u8,4u8,0u8,2u8,4u8,3u8,0u8,51u8,0u8,74u8,0u8,72u8,17u8,236u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,0u8,29u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_curl_shaped()
    ensures choice_spec(curl_shaped(), 4588),
{
    assert(choice_spec(curl_shaped(), 4588)) by (compute)
}
pub open spec fn openssl_shaped() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,107u8,1u8,0u8,0u8,103u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,3u8,1u8,0u8,0u8,60u8,0u8,10u8,0u8,6u8,0u8,4u8,0u8,23u8,0u8,29u8,0u8,13u8,0u8,4u8,0u8,2u8,4u8,3u8,0u8,51u8,0u8,38u8,0u8,36u8,0u8,23u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_openssl_shaped()
    ensures choice_spec(openssl_shaped(), 23),
{
    assert(choice_spec(openssl_shaped(), 23)) by (compute)
}
pub open spec fn aes_only() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,105u8,1u8,0u8,0u8,101u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,1u8,1u8,0u8,0u8,58u8,0u8,10u8,0u8,4u8,0u8,2u8,0u8,29u8,0u8,13u8,0u8,4u8,0u8,2u8,4u8,3u8,0u8,51u8,0u8,38u8,0u8,36u8,0u8,29u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_aes_only()
    ensures forall|g: u16| !choice_spec(aes_only(), g),
{
    assert forall|g: u16| !choice_spec(aes_only(), g) by {
        assert(!choice_spec(aes_only(), g)) by (compute)
    }
}
pub open spec fn unsupported_only() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,105u8,1u8,0u8,0u8,101u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,3u8,1u8,0u8,0u8,58u8,0u8,10u8,0u8,4u8,0u8,2u8,0u8,24u8,0u8,13u8,0u8,4u8,0u8,2u8,4u8,3u8,0u8,51u8,0u8,38u8,0u8,36u8,0u8,24u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_unsupported_only()
    ensures forall|g: u16| !choice_spec(unsupported_only(), g),
{
    assert forall|g: u16| !choice_spec(unsupported_only(), g) by {
        assert(!choice_spec(unsupported_only(), g)) by (compute)
    }
}
pub open spec fn missing_share() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,105u8,1u8,0u8,0u8,101u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,3u8,1u8,0u8,0u8,58u8,0u8,10u8,0u8,4u8,0u8,2u8,0u8,29u8,0u8,13u8,0u8,4u8,0u8,2u8,4u8,3u8,0u8,51u8,0u8,38u8,0u8,36u8,0u8,24u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_missing_share()
    ensures forall|g: u16| !choice_spec(missing_share(), g),
{
    assert forall|g: u16| !choice_spec(missing_share(), g) by {
        assert(!choice_spec(missing_share(), g)) by (compute)
    }
}
pub open spec fn missing_signature() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,105u8,1u8,0u8,0u8,101u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,3u8,1u8,0u8,0u8,58u8,0u8,10u8,0u8,4u8,0u8,2u8,0u8,29u8,0u8,13u8,0u8,4u8,0u8,2u8,8u8,4u8,0u8,51u8,0u8,38u8,0u8,36u8,0u8,29u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_missing_signature()
    ensures forall|g: u16| !choice_spec(missing_signature(), g),
{
    assert forall|g: u16| !choice_spec(missing_signature(), g) by {
        assert(!choice_spec(missing_signature(), g)) by (compute)
    }
}
pub open spec fn hybrid_preference() -> Seq<u8> { seq![22u8,3u8,1u8,0u8,145u8,1u8,0u8,0u8,141u8,3u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,19u8,3u8,1u8,0u8,0u8,98u8,0u8,10u8,0u8,8u8,0u8,6u8,17u8,236u8,0u8,29u8,0u8,23u8,0u8,13u8,0u8,4u8,0u8,2u8,4u8,3u8,0u8,51u8,0u8,74u8,0u8,72u8,0u8,29u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,0u8,23u8,0u8,32u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8,7u8] }
pub proof fn witness_hybrid_preference()
    ensures choice_spec(hybrid_preference(), 29),
{
    assert(choice_spec(hybrid_preference(), 29)) by (compute)
}
}
