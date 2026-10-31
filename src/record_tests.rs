//! Deterministic differential tests. Failures report case numbers, never bytes.
use super::*;

fn random(state: &mut u64) -> usize {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state as usize
}

fn compare_unpad(bytes: &[u8], case: usize) {
    let mut old = bytes.to_vec();
    let mut new = bytes.to_vec();
    let old = InboundOpaqueMessage::new(ContentType::ApplicationData, ProtocolVersion::TLSv1_2, &mut old)
        .into_tls13_unpadded_message();
    let new = unpad_message(InboundOpaqueMessage::new(ContentType::ApplicationData, ProtocolVersion::TLSv1_2, &mut new));
    let same = match (old, new) {
        (Ok(a), Ok(b)) => a.typ == b.typ && a.version == b.version && a.payload == b.payload,
        (Err(a), Err(b)) => a == b,
        _ => false,
    };
    assert!(same, "unpad case {case}");
}

#[test]
fn stage15_unpad_equivalence_100000() {
    let mut seed = 0x548b_019d_763a_2617;
    let lengths = [0, 1, 16_384, 16_385, 16_386, 16_385 + 257];
    for case in 0..100_000 {
        let len = if case < 6 * 256 * 4 { lengths[(case / 256) % 6] }
            else { random(&mut seed) % 17_000 };
        let mut bytes = vec![0; len];
        match (case / (6 * 256)) % 4 {
            0 => {},
            1 => { if len > 0 { bytes[len - 1] = (case % 256) as u8; } },
            2 => { if len > 0 { bytes[len / 2] = (case % 256) as u8; } },
            _ => { for byte in &mut bytes { *byte = random(&mut seed) as u8; } },
        }
        compare_unpad(&bytes, case);
    }
}

#[test]
fn stage15_aad_equivalence_80001() {
    let mut seed = 0x8712_1123_5aa9_436b;
    for len in 0..=70_000 {
        assert!(record::aad(len) == make_tls13_aad(len), "AAD bounded length {len}");
    }
    for case in 0..10_000 {
        let len = random(&mut seed);
        assert!(record::aad(len) == make_tls13_aad(len), "AAD random case {case}");
    }
}

#[test]
fn stage15_inner_plaintext_exact_capacity() {
    for len in [0, 1, 255, 16_384] {
        let payload = vec![7; len];
        let inner = record::inner_plaintext(&payload, 23).unwrap();
        assert_eq!(inner.capacity(), payload.len() + 1, "capacity for length {len}");
    }
}

#[test]
fn stage15_inner_and_length_equivalence() {
    let mut seed = 0x879a_7841_786f_2b17;
    let mut cipher = RecordCipher { key: AeadKey::from([7; 32]), iv: Iv::from([3; 12]) };
    for case in 0..10_000 {
        let len = if case < 256 { 0 } else if case < 512 { 16_384 } else { random(&mut seed) % 16_385 };
        let typ = (case % 256) as u8;
        let mut payload = vec![0; len];
        for b in &mut payload { *b = random(&mut seed) as u8; }
        let chunks = [&payload[..len / 2], &payload[len / 2..]];
        let msg = OutboundPlainMessage { typ: ContentType::from(typ), version: ProtocolVersion::TLSv1_2,
            payload: OutboundChunks::new(&chunks) };
        let mut old = Vec::with_capacity(len + 1);
        msg.payload.copy_to_vec(&mut old);
        old.push(typ);
        let new = record::inner_plaintext(&payload, typ);
        assert!(new.as_ref() == Some(&old), "inner case {case}");
        assert!(cipher.encrypted_payload_len(len) == len + 17, "length case {case}");
        // Compare the actual encrypter's record against the former construction.
        let mut expected = vec![0; len + 17];
        libcrux_chacha20poly1305::encrypt(&[7; 32], &old, &mut expected,
            &make_tls13_aad(len + 17), &aead::nonce(&[3; 12], case as u64).unwrap()).unwrap();
        let encrypted = cipher.encrypt(msg, case as u64).unwrap();
        assert!(encrypted.typ == ContentType::ApplicationData && encrypted.version == ProtocolVersion::TLSv1_2
            && encrypted.payload.as_ref() == expected.as_slice(), "encrypted case {case}");
    }
    for case in 0..10_000 {
        let len = random(&mut seed) % (usize::MAX - 16);
        assert!(cipher.encrypted_payload_len(len) == len + 17, "random length case {case}");
    }
    for distance in 0..=18 {
        let len = usize::MAX - distance;
        assert!(cipher.encrypted_payload_len(len) == len.saturating_add(17), "length boundary {distance}");
    }
}

#[test]
fn stage15_key_shape_and_offers_equivalence() {
    let mut seed = 0x5838_2abe_1234_8712;
    for len in 0..=100 {
        for prefix in 0..=255 {
            let mut bytes = vec![prefix; len];
            for b in bytes.iter_mut().skip(1) { *b = random(&mut seed) as u8; }
            let old: Option<&[u8; 32]> = bytes.as_slice().try_into().ok();
            let new = record::x25519_peer(&bytes);
            assert!(old == new.as_ref(), "key case {len}/{prefix}");
            assert!(record::p256_point_shape(&bytes) == (len == 65 && bytes[0] == 4), "point case {len}/{prefix}");
        }
    }
    for case in 0..10_000 {
        let len = random(&mut seed) % 101;
        let mut offered: Vec<SignatureScheme> = (0..len).map(|_| SignatureScheme::from(random(&mut seed) as u16)).collect();
        if case % 2 == 0 && len > 0 { offered[len / 2] = SignatureScheme::ECDSA_NISTP256_SHA256; }
        let raw: Vec<u16> = offered.iter().copied().map(u16::from).collect();
        assert!(record::offers(&raw, 0x0403) == offered.contains(&SignatureScheme::ECDSA_NISTP256_SHA256), "scheme case {case}");
        let wanted = random(&mut seed) as u16;
        assert!(record::offers(&raw, wanted) == raw.contains(&wanted), "offers case {case}");
    }
}
