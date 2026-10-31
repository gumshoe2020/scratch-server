use std::io;
use transport_core::io::{self as core, ReadResult, Step};

// Copied from the old shell match, returning a comparable event and start flag.
fn old_classify(r: &io::Result<usize>) -> (Step, bool) {
    match r {
        Ok(0) => (Step::Eof, false),
        Ok(n) => (Step::Bytes(*n), true),
        Err(e) if e.kind() == io::ErrorKind::Interrupted => (Step::Retry, false),
        Err(e) if matches!(e.kind(), io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock) =>
            (Step::ReadTimeout, false),
        Err(_) => (Step::Fail, false),
    }
}
#[test]
fn stage16_classification_all_error_kinds() {
    // Every stable ErrorKind variant in the pinned Rust 1.99 toolchain.
    use io::ErrorKind::*;
    let kinds = [NotFound, PermissionDenied, ConnectionRefused, ConnectionReset,
        HostUnreachable, NetworkUnreachable, ConnectionAborted, NotConnected,
        AddrInUse, AddrNotAvailable, NetworkDown, BrokenPipe, AlreadyExists,
        WouldBlock, NotADirectory, IsADirectory, DirectoryNotEmpty, ReadOnlyFilesystem,
        StaleNetworkFileHandle, InvalidInput, InvalidData, TimedOut, WriteZero,
        StorageFull, NotSeekable, QuotaExceeded, FileTooLarge, ResourceBusy,
        ExecutableFileBusy, Deadlock, CrossesDevices, TooManyLinks, InvalidFilename,
        ArgumentListTooLong, Interrupted, Unsupported, UnexpectedEof, OutOfMemory, Other];
    let mut observed = std::collections::BTreeSet::new();
    for r in kinds.into_iter().map(|k| Err(io::Error::from(k)))
        .chain((-1..=4096).map(|errno| Err(io::Error::from_raw_os_error(errno))))
        .chain([Ok(0), Ok(1), Ok(1024)]) {
        if let Err(ref error) = r { observed.insert(format!("{:?}", error.kind())); }
        assert_eq!(core::classify(super::read_result(&r)), old_classify(&r));
    }
    // Unstable-named enum variants are obtainable through stable errno mapping.
    for name in ["FilesystemLoop", "InProgress", "InputOutputError", "TooManyOpenFiles", "Uncategorized"] {
        assert!(observed.contains(name), "missing error kind {name}");
    }
    eprintln!("stage16 classified error kinds: {observed:?}");
    assert_eq!(observed.len(), 44);
    assert_eq!(core::classify(ReadResult::Data(usize::MAX)), (Step::Bytes(usize::MAX), true));
}
fn random(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
// Old checked_duration_since(...).filter(!is_zero) and started branch,
// evaluated in Duration space so MAX ns need not fit a platform Instant.
fn old_timeout(elapsed: u64, started: bool) -> Option<u64> {
    use std::time::Duration;
    if started { return Some(core::READ_NS); }
    Duration::from_nanos(core::IDLE_NS)
        .checked_sub(Duration::from_nanos(elapsed))
        .filter(|d| !d.is_zero()).map(|d| d.as_nanos() as u64)
}
#[test]
fn stage16_deadline_equivalence_100000() {
    let edges = [0, core::IDLE_NS - 1, core::IDLE_NS, core::IDLE_NS + 1,
        u64::MAX, core::READ_NS, core::REQUEST_NS, u64::MAX - core::REQUEST_NS];
    let mut seed = 0x84ab39821u64;
    for case in 0..100_000 {
        let elapsed = if case < 128 { edges[(case / 16) % 8] }
            else if case % 2 == 0 { random(&mut seed) % (30 * core::IDLE_NS) }
            else { random(&mut seed) };
        let first = if case % 2 == 0 { None } else {
            Some(if case < 128 { edges[(case / 2) % 8] } else { random(&mut seed).min(elapsed) })
        };
        let old = old_timeout(elapsed, first.is_some());
        // Disabling only the added request deadline leaves the new idle
        // computation and the inherited started timeout exactly equivalent.
        let disabled = if first.is_some() { Some(core::READ_NS) }
            else { core::read_timeout(elapsed, None) };
        assert_eq!(disabled, old);
        let enabled = core::read_timeout(elapsed, first);
        match first {
            None => assert_eq!(enabled, old),
            Some(t) => {
                let deadline = t.saturating_add(core::REQUEST_NS);
                if elapsed >= deadline { assert_eq!(enabled, None); }
                else {
                    assert_eq!(enabled, Some(core::READ_NS.min(deadline - elapsed)));
                    assert!(enabled.unwrap() <= old.unwrap());
                }
            }
        }
    }
}
fn compare_port(bytes: &[u8]) {
    let old = std::str::from_utf8(bytes).ok().and_then(|s| s.parse::<u16>().ok());
    // Input bytes are deliberately excluded from failure output.
    assert_eq!(core::parse_port(Some(bytes)), old, "port result mismatch");
}
#[test]
fn stage16_port_exhaustive_and_random() {
    let alphabet = b"0123456789+- a\0";
    let mut cases = 0usize;
    for len in 0..=6u32 {
        let total = alphabet.len().pow(len);
        for mut number in 0..total {
            let mut bytes = [0u8; 6];
            for b in &mut bytes[..len as usize] {
                *b = alphabet[number % alphabet.len()];
                number /= alphabet.len();
            }
            compare_port(&bytes[..len as usize]);
            cases += 1;
        }
    }
    assert_eq!(cases, 12_204_241);
    let mut seed = 0x3298febau64;
    for _ in 0..10_000 {
        let len = (random(&mut seed) % 100) as usize;
        let bytes: Vec<_> = (0..len).map(|_| random(&mut seed) as u8).collect();
        compare_port(&bytes);
    }
    for bytes in [b"0".as_slice(), b"65535", b"65536", b"+0", b"+65535", b"00080", b""] {
        compare_port(bytes);
    }
    assert_eq!(core::parse_port(None), Some(8080));
}
