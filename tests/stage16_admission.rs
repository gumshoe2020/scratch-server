//! Stage 16 stop-rule evidence. The approved fix preserves old behavior except the documented MAX overflow.

// Verbatim closure body from src/main.rs at 5ca8b97 (also at 6860b91).
fn old_admit(n: usize) -> Option<usize> {
    (n < 64).then_some(n + 1)
}

#[test]
fn stage16_admission_equivalence_0_through_200() {
    for active in 0..=200 {
        assert_eq!(old_admit(active), transport_core::io::admit(active));
    }
}

#[test]
fn stage16_admission_max_exposes_old_overflow() {
    let old = std::panic::catch_unwind(|| old_admit(usize::MAX));
    let specified = transport_core::io::admit(usize::MAX);
    assert_eq!(specified, None);
    if cfg!(debug_assertions) {
        // then_some evaluates its argument even when the condition is false.
        // Debug builds panic instead of returning the specified rejection.
        assert!(old.is_err(), "old admission unexpectedly did not overflow");
    } else {
        // The shipped release profile disables overflow checks. It wraps the
        // unused argument and returns None; no live exploit is demonstrated.
        assert_eq!(old.unwrap(), specified);
    }
}

#[test]
fn stage16_atomic_admission_and_release() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use transport_core::io::{admit, release};
    for active in (0..=200).chain([usize::MAX]) {
        let counter = AtomicUsize::new(active);
        let result = counter.fetch_update(Ordering::Relaxed, Ordering::Relaxed, admit);
        if active < 64 {
            assert_eq!(result, Ok(active));
            assert!(counter.load(Ordering::Relaxed) <= 64);
            counter.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| Some(release(n))).unwrap();
            assert_eq!(counter.load(Ordering::Relaxed), active);
        } else {
            assert_eq!(result, Err(active));
            assert_eq!(counter.load(Ordering::Relaxed), active);
        }
        assert_eq!(release(active), active.saturating_sub(1));
    }
}
