//! Stage-13 implementations retained verbatim as test-only stage-14 oracles.
//! Do not tighten this walk: malformed-input equivalence is part of the spec.
use super::*;
use std::time::Duration;
use transport_core::http::{classify, drop_header};

#[derive(Default)]
struct UnknownLog {
    names: Vec<(String, Instant)>,
}
impl UnknownLog {
    fn should_log(&mut self, name: &[u8], limit: u16, now: Instant) -> bool {
        self.names
            .retain(|(_, time)| now.duration_since(*time) < Duration::from_secs(60));
        if self.names.len() >= usize::from(limit).min(64)
            || self
                .names
                .iter()
                .any(|(old, _)| old.as_bytes().eq_ignore_ascii_case(name))
        {
            return false;
        }
        self.names
            .push((String::from_utf8_lossy(name).to_ascii_lowercase(), now));
        true
    }
}
fn sanitized(name: &[u8]) -> String {
    name.iter()
        .copied()
        .filter(|b| b.is_ascii_alphanumeric() || *b == b'-')
        .take(64)
        .map(char::from)
        .collect()
}
// Name-only diagnostic walk. Its output never controls parser acceptance.
fn header_names(bytes: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut rest = bytes;
    let mut first = true;
    std::iter::from_fn(move || loop {
        let end = rest.windows(2).position(|w| w == b"\r\n")?;
        let line = &rest[..end];
        rest = &rest[end + 2..];
        if first {
            first = false;
            continue;
        }
        if line.is_empty() {
            rest = &[];
            return None;
        }
        if let Some(colon) = line.iter().position(|b| *b == b':') {
            return Some(&line[..colon]);
        }
    })
}
fn rejection_line(bytes: &[u8], policy: &Policy) -> String {
    let mut host = false;
    let mut connection = false;
    let mut count = 0;
    for name in header_names(bytes) {
        count += 1;
        if count > 24 {
            return "status=400 reason=too_many_headers".to_owned();
        }
        let class = classify(name);
        let reason = if (19..28).contains(&class) {
            Some("hard_reject")
        } else if (class == 0 && host) || (class == 2 && connection) {
            Some("duplicate")
        } else if !drop_header(policy, class) {
            Some(if class < 44 {
                "guarded"
            } else {
                "unknown_header"
            })
        } else {
            None
        };
        host |= class == 0;
        connection |= class == 2;
        if let Some(reason) = reason {
            return format!("status=400 reason={reason} name={}", sanitized(name));
        }
    }
    "status=400 reason=grammar".to_owned()
}

#[test]
fn stage14_compatibility_header_spans_contain_bare_line_breaks() {
    for separator in [b'\r', b'\n'] {
        let mut wire = b"GET / HTTP/1.1\r\nX".to_vec();
        wire.push(separator);
        wire.extend_from_slice(b"Name: x\r\n\r\n");
        let names: Vec<_> = header_names(&wire).collect();
        assert!(names.len() == 1);
        assert!(names[0].contains(&separator));
        assert!(super::header_names(&wire).eq(names.into_iter()));
        // Amended property 3 preserves bare CR/LF; property 4 sanitizes log output.
    }
}

#[test]
fn stage14_compatibility_malformed_name_changes_rejection_diagnostic() {
    let strict = Policy {
        unknown_drop: false,
        guarded_drop: 0,
        log_limit: 60,
    };
    for separator in [b'\r', b'\n'] {
        let mut wire = b"GET / HTTP/1.1\r\nX".to_vec();
        wire.push(separator);
        wire.extend_from_slice(b"Name: x\r\n\r\n");
        let old = rejection_line(&wire, &strict);
        assert!(old == "status=400 reason=unknown_header name=XName");
        assert!(super::rejection_line(&wire, &strict) == old);
        assert!(transport_core::http::parse_policy(&wire, &strict).is_none());
        // Preserve this malformed-name diagnostic exactly under resolution A.
        assert!(old != "status=400 reason=grammar");
    }
}

#[test]
fn stage14_preserved_oracle_sanitizer_and_limiter() {
    let now = Instant::now();
    for byte in 0..=255u8 {
        assert!(sanitized(&[byte]) == super::sanitized(&[byte]));
    }
    for limit in [0, 1, 60, 600] {
        let mut old = UnknownLog::default();
        let mut current = app::UnknownLog::new();
        for i in 0..1000 {
            let name = format!("X-{}", i % 100);
            assert!(
                old.should_log(name.as_bytes(), limit, now)
                    == current.should_log(name.as_bytes(), limit, 0)
            );
        }
        for seconds in [59, 60] {
            assert!(
                old.should_log(b"x-0", limit, now + Duration::from_secs(seconds))
                    == current.should_log(b"x-0", limit, seconds * 1000)
            );
        }
    }
}

fn next_random(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

#[test]
fn stage14_header_and_diagnostic_equivalence_100000_blocks() {
    let mut seeds: Vec<Vec<u8>> = [
        include_bytes!("../tests/safari-https-root.http").as_slice(),
        include_bytes!("../tests/firefox-https-root.http").as_slice(),
        include_bytes!("../tests/brave-https-root.http").as_slice(),
        include_bytes!("../tests/brave-cache-disabled-https-root.http").as_slice(),
        include_bytes!("../tests/brave-dnt-https-root.http").as_slice(),
    ]
    .into_iter()
    .map(<[u8]>::to_vec)
    .collect();
    for line in include_str!("../tests/malformed.hex").lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        seeds.push(
            line.split_whitespace()
                .map(|hex| u8::from_str_radix(hex, 16).unwrap())
                .collect(),
        );
    }
    let policies = [
        Policy::builtin(),
        Policy {
            unknown_drop: false,
            guarded_drop: 0,
            log_limit: 60,
        },
        Policy {
            unknown_drop: true,
            guarded_drop: u16::MAX,
            log_limit: 600,
        },
    ];
    let mut seed = 0x1414_cafe_1234_5678;
    for case in 0..100_000 {
        let mut wire = seeds[next_random(&mut seed) as usize % seeds.len()].clone();
        match case % 14 {
            0 => (),
            1 => {
                if !wire.is_empty() {
                    let i = next_random(&mut seed) as usize % wire.len();
                    wire[i] = next_random(&mut seed) as u8;
                }
            }
            2..=4 => {
                let at = wire.iter().position(|c| *c == b':').unwrap_or(wire.len());
                wire.insert(
                    at,
                    match case % 14 {
                        2 => b'\r',
                        3 => b'\n',
                        _ => 0,
                    },
                );
            }
            5 => {
                let n = next_random(&mut seed) as usize % (wire.len() + 1);
                wire.truncate(n);
            }
            6 => {
                wire = b"GET / HTTP/1.1\r\n".to_vec();
                let n = next_random(&mut seed) as usize % 301;
                wire.extend((0..n).map(|_| next_random(&mut seed) as u8));
                wire.extend_from_slice(b": x\r\n\r\n");
            }
            7 => wire = b"GET / HTTP/1.1\r\n: x\r\nOdd:: x\r\n\r\n".to_vec(),
            8 => {
                wire = b"GET / HTTP/1.1\r\n".to_vec();
                for _ in 0..(25 + next_random(&mut seed) as usize % 10) {
                    wire.extend_from_slice(b"Accept: x\r\n");
                }
                wire.extend_from_slice(b"\r\n");
            }
            9 => wire.resize(4097, b'X'),
            10 => {
                let at = next_random(&mut seed) as usize % (wire.len() + 1);
                wire.insert(at, b':');
            }
            11 => wire.extend_from_slice(b"\r\n\r\nLate: x\r\n\r\n"),
            12 => wire = b"GET / HTTP/1.1\r\nX\xff\xfe: x\r\n\r\n".to_vec(),
            _ => {
                let n = next_random(&mut seed) as usize % 4098;
                wire = (0..n).map(|_| next_random(&mut seed) as u8).collect();
            }
        }
        let old: Vec<_> = header_names(&wire).collect();
        let new: Vec<_> = super::header_names(&wire).collect();
        assert!(
            old == new,
            "header sequence mismatch at generated case {case}"
        );
        for policy in &policies {
            assert!(
                rejection_line(&wire, policy) == super::rejection_line(&wire, policy),
                "diagnostic mismatch at generated case {case}"
            );
        }
    }
}

#[test]
fn stage14_sanitizer_equivalence_random_names() {
    let mut seed = 0x1414_abcd_1234_9876;
    for byte in 0..=255u8 {
        assert!(sanitized(&[byte]).as_bytes() == app::sanitize(&[byte]));
    }
    for case in 0..10_000 {
        let length = next_random(&mut seed) as usize % 301;
        let name: Vec<_> = (0..length).map(|_| next_random(&mut seed) as u8).collect();
        assert!(
            sanitized(&name).as_bytes() == app::sanitize(&name),
            "sanitizer mismatch at generated case {case}"
        );
    }
}

#[test]
fn stage14_limiter_equivalence_sliding_windows() {
    let epoch = Instant::now();
    for limit in [0, 1, 60, 600] {
        let mut old = UnknownLog::default();
        let mut new = app::UnknownLog::new();
        let mut emitted = Vec::new();
        for call in 0..20_000u64 {
            let now_ms = call * 137;
            let name = if call % 2 == 0 {
                format!("X-{}", call % 100)
            } else {
                format!("x-{}", call % 100)
            };
            let expected = old.should_log(
                name.as_bytes(),
                limit,
                epoch + Duration::from_millis(now_ms),
            );
            let actual = new.should_log(name.as_bytes(), limit, now_ms);
            assert!(
                actual == expected,
                "limiter mismatch at call {call}, limit {limit}"
            );
            if actual {
                emitted.push(now_ms);
            }
            let count = emitted
                .iter()
                .rev()
                .take_while(|t| now_ms - **t < 60_000)
                .count();
            assert!(count <= usize::from(limit).min(64));
            assert!(new.entries.len() <= usize::from(limit).min(64));
        }
    }
    // Rollback keeps future entries; saturated age is zero in the old limiter too.
    let mut old = UnknownLog::default();
    let mut new = app::UnknownLog::new();
    for now_ms in [60_001, 1, 0, 60_000, 120_001, u64::MAX] {
        // Instant cannot represent u64::MAX ms on every platform; the pure core can.
        if now_ms != u64::MAX {
            assert!(
                old.should_log(b"X-Rollback", 60, epoch + Duration::from_millis(now_ms))
                    == new.should_log(b"X-Rollback", 60, now_ms)
            );
        } else {
            assert!(new.should_log(b"X-Rollback", 60, now_ms));
        }
    }
}
