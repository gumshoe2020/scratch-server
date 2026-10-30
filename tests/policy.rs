//! Policy configuration, startup refusal, name grammar and isolation acceptance.
use transport_core::http::{classify, drop_header, parse_config, parse_policy, Policy};

const HARD: &[&str] = &[
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
const GUARDED: &[&str] = &[
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

fn malformed() -> Vec<Vec<u8>> {
    vec![
        b"surprise drop".to_vec(),
        b"unknown use".to_vec(),
        b"unknown drop extra".to_vec(),
        b"guarded Content-Length drop".to_vec(),
        b"guarded Transfer-Encoding drop".to_vec(),
        b"guarded Host drop".to_vec(),
        b"guarded Unlisted drop".to_vec(),
        b"guarded Cookie drop\nguarded cOoKiE reject".to_vec(),
        b"unknown drop\nunknown reject".to_vec(),
        b"log-unknown-per-minute 1\nlog-unknown-per-minute 2".to_vec(),
        b"log-unknown-per-minute 601".to_vec(),
        b"log-unknown-per-minute -1".to_vec(),
        b"log-unknown-per-minute 1x".to_vec(),
        b"log-unknown-per-minute 99999999999999999999".to_vec(),
        vec![b'#', 0xff],
        "# nonascii é".as_bytes().to_vec(),
        vec![0],
        vec![b' '; 4097],
        vec![b'\n'; 65],
        b"guarded Cookie".to_vec(),
    ]
}
#[test]
fn malformed_configs_fail_before_bind_or_tls_load() {
    let directory = std::env::current_dir()
        .unwrap()
        .join(format!("target/policy-startup-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    for (i, input) in malformed().into_iter().enumerate() {
        assert!(
            parse_config(&input).is_none(),
            "malformed config {i} loaded"
        );
        let path = directory.join(format!("{i}.conf"));
        std::fs::write(&path, input).unwrap();
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_scratch-server"))
            .env("SCRATCH_HEADER_POLICY", path)
            .env("SCRATCH_TLS_DIR", directory.join("absent"))
            .env("SCRATCH_PORT", "not-a-port")
            .output()
            .unwrap();
        assert!(!output.status.success(), "startup case {i}");
        assert!(output.stdout.is_empty(), "startup case {i} printed config");
        assert!(
            output.stderr == b"server: invalid header policy\n",
            "startup case {i} failed for another reason"
        );
    }
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_scratch-server"))
        .env("SCRATCH_HEADER_POLICY", directory.join("missing"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr == b"server: header policy load failed\n");
    std::fs::remove_dir_all(directory).unwrap();
}
#[test]
fn ten_thousand_generated_configs_cannot_loosen_hard_reject() {
    let mut seed = 0x51f3_d8c2_9917_601du64;
    for i in 0..10_000 {
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let choice = next() % 7;
        let mut config = match choice {
            0 => format!(
                "unknown {}\nlog-unknown-per-minute {}\n",
                if next() & 1 == 0 { "drop" } else { "reject" },
                next() % 601
            )
            .into_bytes(),
            1 => format!(
                "guarded {} drop\n",
                GUARDED[next() as usize % GUARDED.len()]
            )
            .into_bytes(),
            2 => format!("guarded {} drop\n", HARD[next() as usize % HARD.len()]).into_bytes(),
            3 => format!("unknown drop\nunknown reject\n# {i}\n").into_bytes(),
            4 => (0..next() % 5000).map(|_| next() as u8).collect(),
            5 => format!(
                "# synthetic comment\r\n\tguarded {} reject # comment\r\nunknown reject\r\n",
                GUARDED[next() as usize % GUARDED.len()]
            )
            .into_bytes(),
            _ => format!("log-unknown-per-minute {}", next()).into_bytes(),
        };
        if i % 11 == 0 && !config.is_empty() {
            let at = next() as usize % config.len();
            config[at] = next() as u8;
        }
        if let Some(policy) = parse_config(&config) {
            assert!(policy.log_limit <= 600);
            for name in HARD {
                let class = classify(name.as_bytes());
                assert!(!drop_header(&policy, class));
                let wire = format!("GET / HTTP/1.1\r\n{name}: synthetic\r\n\r\n");
                assert!(parse_policy(wire.as_bytes(), &policy).is_none());
            }
        }
    }
}
#[test]
fn defaults_guarded_overrides_token_and_count_boundaries() {
    let default = Policy::builtin();
    assert_eq!(
        parse_config(include_bytes!("../config/header-policy.conf")).unwrap(),
        default
    );
    let strict = parse_config(include_bytes!("../config/header-policy-strict.conf")).unwrap();
    assert!(strict.is_strict());
    assert!(parse_config(&[b'\n'; 64]).is_some());
    assert!(parse_config(&[b' '; 4096]).is_some());
    assert!(parse_config(b"log-unknown-per-minute 0").unwrap().log_limit == 0);
    assert!(
        parse_config(b"log-unknown-per-minute 600")
            .unwrap()
            .log_limit
            == 600
    );
    for (i, name) in GUARDED.iter().enumerate() {
        let config = format!("guarded {} drop", name.to_ascii_uppercase());
        let policy = parse_config(config.as_bytes()).unwrap();
        assert_eq!(policy.guarded_drop, 1 << i);
        let wire = format!("GET / HTTP/1.1\r\n{name}: synthetic\r\n\r\n");
        assert!(parse_policy(wire.as_bytes(), &default).is_none());
        assert!(parse_policy(wire.as_bytes(), &policy).is_some());
    }
    for (name, accepted) in [
        ("!#$%&'*+-.^_`|~".to_owned(), true),
        ("A".repeat(64), true),
        ("A".repeat(65), false),
        ("".to_owned(), false),
        ("X Y".to_owned(), false),
        ("X/Y".to_owned(), false),
        ("X:Y".to_owned(), false),
    ] {
        let wire = format!("GET / HTTP/1.1\r\n{name}: synthetic\r\n\r\n");
        assert_eq!(parse_policy(wire.as_bytes(), &default).is_some(), accepted);
    }
    for count in [24, 25] {
        let wire = format!(
            "GET / HTTP/1.1\r\n{}\r\n",
            "Accept: synthetic\r\n".repeat(count)
        );
        assert_eq!(
            parse_policy(wire.as_bytes(), &default).is_some(),
            count == 24
        );
    }
}
