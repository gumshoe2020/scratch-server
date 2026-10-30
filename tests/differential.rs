//! Independent grammar recognition, plus httparse as the accepted-field oracle.
use http_subset::{parse_policy, Policy};
use proptest::prelude::*;

type Fields = (String, String, Vec<(String, Vec<u8>)>);

// Do not use production parser helpers/constants here: this is a second grammar
// implementation, from PROOFS.md, so a shared scanning bug can be detected.
fn grammar(bytes: &[u8], policy: &Policy) -> Option<Fields> {
    if bytes.len() > 4096 || !bytes.is_ascii() {
        return None;
    }
    let text = std::str::from_utf8(bytes).ok()?.strip_suffix("\r\n\r\n")?;
    let mut lines = text.split("\r\n");
    let mut line = lines.next()?.split(' ');
    let method = line.next()?;
    let path = line.next()?;
    let version = line.next()?;
    if line.next().is_some()
        || !["GET", "HEAD"].contains(&method)
        || !["HTTP/1.0", "HTTP/1.1"].contains(&version)
        || !path.starts_with('/')
        || path.len() > 256
        || !path
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"/-_.".contains(&b))
    {
        return None;
    }
    let mut headers = vec![];
    for line in lines {
        let (name, value) = line.split_once(": ")?;
        let known = [
            "Host",
            "Accept",
            "Connection",
            "User-Agent",
            "Accept-Language",
            "Accept-Encoding",
            "Upgrade-Insecure-Requests",
            "Priority",
            "Referer",
            "Sec-Fetch-Dest",
            "Sec-Fetch-Site",
            "Sec-Fetch-Mode",
            "Sec-Fetch-User",
            "Cache-Control",
            "Sec-GPC",
            "Sec-CH-UA",
            "Sec-CH-UA-Mobile",
            "Sec-CH-UA-Platform",
            "Pragma",
        ]
        .iter()
        .any(|allowed| name.eq_ignore_ascii_case(allowed));
        let hard = [
            "Content-Length",
            "Transfer-Encoding",
            "TE",
            "Trailer",
            "Expect",
            "Upgrade",
            "HTTP2-Settings",
            "Proxy-Connection",
            "Keep-Alive",
        ];
        let guarded = [
            "Cookie",
            "Authorization",
            "Proxy-Authorization",
            "Range",
            "If-Range",
            "If-Match",
            "If-None-Match",
            "If-Modified-Since",
            "If-Unmodified-Since",
            "Forwarded",
            "X-Forwarded-For",
            "X-Forwarded-Host",
            "X-Forwarded-Proto",
            "X-Real-IP",
            "Via",
            "Max-Forwards",
        ];
        let guard = guarded.iter().position(|n| name.eq_ignore_ascii_case(n));
        if name.is_empty()
            || name.len() > 64
            || !name.bytes().all(|b| {
                b.is_ascii_alphanumeric()
                    || b"!#$%&' *+-.^_`|~"
                        .iter()
                        .copied()
                        .filter(|c| *c != b' ')
                        .any(|c| c == b)
            })
            || hard.iter().any(|n| name.eq_ignore_ascii_case(n))
            || guard.is_some_and(|i| policy.guarded_drop & (1 << i) == 0)
            || (!known && guard.is_none() && !policy.unknown_drop)
            || ((name.eq_ignore_ascii_case("Host") || name.eq_ignore_ascii_case("Connection"))
                && headers
                    .iter()
                    .any(|(n, _): &(String, Vec<u8>)| name.eq_ignore_ascii_case(n)))
            || value.is_empty()
            || value.len() > 256
            || value.starts_with(' ')
            || value.ends_with(' ')
            || !value.bytes().all(|b| (32..=126).contains(&b))
        {
            return None;
        }
        headers.push((name.to_owned(), value.as_bytes().to_vec()));
    }
    if headers.len() > 24 {
        return None;
    }
    Some((method.to_owned(), path.to_owned(), headers))
}

fn compare_policy(bytes: &[u8], policy: &Policy) {
    let expected = grammar(bytes, policy);
    let actual = parse_policy(bytes, policy);
    match expected {
        None => assert!(actual.is_none(), "outside grammar accepted"),
        Some((method, path, headers)) => {
            let actual = actual.expect("inside grammar rejected");
            assert_eq!(if actual.head { "HEAD" } else { "GET" }, method);
            assert_eq!(&bytes[actual.path_start..actual.path_end], path.as_bytes());
            let mut storage = [httparse::EMPTY_HEADER; 24];
            let mut oracle = httparse::Request::new(&mut storage);
            assert_eq!(
                oracle.parse(bytes).unwrap(),
                httparse::Status::Complete(bytes.len())
            );
            assert_eq!(oracle.method, Some(method.as_str()));
            assert_eq!(oracle.path, Some(path.as_str()));
            assert_eq!(oracle.headers.len(), headers.len());
            for (oracle, (name, value)) in oracle.headers.iter().zip(headers.iter()) {
                assert!(oracle.name == name);
                assert!(oracle.value == value);
            }
        }
    }
}

fn compare(bytes: &[u8]) {
    for policy in [
        Policy::builtin(),
        Policy {
            unknown_drop: false,
            guarded_drop: 0,
            log_limit: 60,
        },
        Policy {
            unknown_drop: true,
            guarded_drop: 1,
            log_limit: 60,
        },
    ] {
        compare_policy(bytes, &policy);
    }
}

fn valid() -> impl Strategy<Value = Vec<u8>> {
    (
        any::<bool>(),
        any::<bool>(),
        "/[a-zA-Z0-9/_.-]{0,255}",
        prop::collection::vec(
            (0usize..24, any::<u32>(), "[!-~]([ -~]{0,30}[!-~])?"),
            0..25,
        ),
    )
        .prop_map(|(head, version, path, headers)| {
            let method = if head { "HEAD" } else { "GET" };
            let version = if version { "1.1" } else { "1.0" };
            let mut wire = format!("{method} {path} HTTP/{version}\r\n");
            for (name, mask, value) in headers {
                wire.push_str(&format!(
                    "{}: {value}\r\n",
                    [
                        "Host",
                        "Accept",
                        "Connection",
                        "User-Agent",
                        "Accept-Language",
                        "Accept-Encoding",
                        "Upgrade-Insecure-Requests",
                        "Priority",
                        "Referer",
                        "Sec-Fetch-Dest",
                        "Sec-Fetch-Site",
                        "Sec-Fetch-Mode",
                        "Sec-Fetch-User",
                        "Cache-Control",
                        "Sec-GPC",
                        "Sec-CH-UA",
                        "Sec-CH-UA-Mobile",
                        "Sec-CH-UA-Platform",
                        "Pragma",
                        "DNT",
                        "X-Custom!",
                        "Content-Length",
                        "Cookie",
                        "Range",
                    ][name]
                        .bytes()
                        .enumerate()
                        .map(|(i, b)| if mask & (1 << i) != 0 {
                            b.to_ascii_lowercase()
                        } else {
                            b.to_ascii_uppercase()
                        })
                        .map(char::from)
                        .collect::<String>()
                ));
            }
            wire.push_str("\r\n");
            wire.into_bytes()
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50_000))]
    #[test]
    fn fifty_thousand_inside_subset(bytes in valid()) {
        compare(&bytes);
    }

    #[test]
    fn fifty_thousand_mutated_or_arbitrary(bytes in prop_oneof![
        4 => (valid(), any::<usize>(), any::<u8>()).prop_map(|(mut b, i, c)| { let i=i % b.len(); b[i]=c; b }),
        2 => (valid(), any::<usize>()).prop_map(|(mut b,i)| { b.truncate(i % b.len()); b }),
        1 => prop::collection::vec(any::<u8>(), 0..5000),
    ]) {
        compare(&bytes);
    }
}

#[test]
fn grammar_boundaries_and_regressions() {
    // Explicit edge cases supplement random generation; no mismatches found yet.
    for size in [0, 1, 255, 256, 257, 4096, 4097] {
        compare(format!("GET /{} HTTP/1.1\r\n\r\n", "a".repeat(size)).as_bytes());
        compare(format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", "x".repeat(size)).as_bytes());
    }
    for count in [0, 1, 16, 17, 23, 24, 25] {
        compare(format!("GET / HTTP/1.1\r\n{}\r\n", "Accept: x\r\n".repeat(count)).as_bytes());
    }
    let mut limit = format!(
        "GET / HTTP/1.1\r\n{}Accept: ",
        format!("Accept: {}\r\n", "x".repeat(256)).repeat(15)
    );
    limit.push_str(&"x".repeat(4096 - limit.len() - 4));
    limit.push_str("\r\n\r\n");
    assert_eq!(limit.len(), 4096);
    assert!(grammar(limit.as_bytes(), &Policy::builtin()).is_some());
    compare(limit.as_bytes());
    limit.insert(limit.len() - 4, 'x');
    assert!(grammar(limit.as_bytes(), &Policy::builtin()).is_none());
    compare(limit.as_bytes());
    for bytes in [
        b"GET / HTTP/1.1\r\nHost: x:y z\r\n\r\n".as_slice(),
        b"HEAD / HTTP/1.0\r\nUser-Agent: ! ~\r\n\r\n",
        b"GET / HTTP/1.1\r\nHost: x\r\n\r\nignored",
    ] {
        compare(bytes);
    }
}

// The amended strict accepted set excludes the two stage-11 witnesses.
#[test]
fn duplicates_reject_for_every_policy() {
    for name in ["Host", "Connection"] {
        let wire = format!(
            "GET / HTTP/1.1\r\n{name}: x\r\n{}: x\r\n\r\n",
            name.to_ascii_lowercase()
        );
        compare(wire.as_bytes());
        assert!(parse_policy(wire.as_bytes(), &Policy::builtin()).is_none());
    }
}
