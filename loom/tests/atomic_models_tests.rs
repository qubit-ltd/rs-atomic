// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use loom::model;
use loom::sync::Arc;
use loom::sync::atomic::AtomicBool;
use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering;
use loom::thread::spawn;
use loom::thread::yield_now;
use qubit_atomic::atomic::testing::try_update_atomic_count_with_ordering;

struct CountPublicationState {
    counter: AtomicUsize,
    data: [AtomicUsize; 2],
}

/// Performs one checked increment through the production `AtomicCount` core.
fn increment_once(counter: &AtomicUsize) {
    assert!(
        try_update_atomic_count_with_ordering(
            |ordering| counter.load(ordering),
            |current, next, success, failure| counter.compare_exchange_weak(current, next, success, failure),
            |current| current.checked_add(1),
        )
        .is_some(),
        "counter increment should not overflow"
    );
}

/// Decrements through the production `AtomicCount` ordering and retry core.
fn decrement_once(counter: &AtomicUsize) -> usize {
    try_update_atomic_count_with_ordering(
        |ordering| counter.load(ordering),
        |old, new, success, failure| counter.compare_exchange_weak(old, new, success, failure),
        |old| old.checked_sub(1),
    )
    .expect("count must remain nonnegative")
}

#[test]
/// Basic Loom release/acquire example; this does not exercise the production
/// wrapper.
fn test_loom_release_acquire_visibility() {
    model(|| {
        let data = Arc::new(AtomicUsize::new(0));
        let ready = Arc::new(AtomicBool::new(false));

        let data_writer = data.clone();
        let ready_writer = ready.clone();
        let writer = spawn(move || {
            data_writer.store(42, Ordering::Relaxed);
            ready_writer.store(true, Ordering::Release);
        });

        let data_reader = data.clone();
        let ready_reader = ready.clone();
        let reader = spawn(move || {
            while !ready_reader.load(Ordering::Acquire) {
                yield_now();
            }
            assert_eq!(data_reader.load(Ordering::Relaxed), 42);
        });

        writer.join().expect("writer thread panicked");
        reader.join().expect("reader thread panicked");
    });
}

#[test]
fn test_loom_checked_update_prevents_underflow() {
    model(|| {
        let counter = Arc::new(AtomicUsize::new(1));

        let c1 = counter.clone();
        let t1 = spawn(move || {
            try_update_atomic_count_with_ordering(
                |ordering| c1.load(ordering),
                |current, next, success, failure| c1.compare_exchange_weak(current, next, success, failure),
                |current| current.checked_sub(1),
            )
        });

        let c2 = counter.clone();
        let t2 = spawn(move || {
            try_update_atomic_count_with_ordering(
                |ordering| c2.load(ordering),
                |current, next, success, failure| c2.compare_exchange_weak(current, next, success, failure),
                |current| current.checked_sub(1),
            )
        });

        let results = [
            t1.join().expect("first decrement thread panicked"),
            t2.join().expect("second decrement thread panicked"),
        ];
        assert_eq!(results.iter().filter(|result| result.is_some()).count(), 1);
        assert_eq!(counter.load(Ordering::Acquire), 0);
    });
}

#[test]
fn test_loom_count_zero_publishes_all_completed_data() {
    model(|| {
        let state = Arc::new(CountPublicationState {
            counter: AtomicUsize::new(2),
            data: [AtomicUsize::new(0), AtomicUsize::new(0)],
        });
        let mut handles = Vec::new();
        for index in 0..2 {
            let state = Arc::clone(&state);
            handles.push(spawn(move || {
                state.data[index].store(1, Ordering::Relaxed);
                if decrement_once(&state.counter) == 0 {
                    assert_eq!(state.data[0].load(Ordering::Relaxed), 1);
                    assert_eq!(state.data[1].load(Ordering::Relaxed), 1);
                }
            }));
        }
        let observed_state = Arc::clone(&state);
        let observer = spawn(move || {
            if observed_state.counter.load(Ordering::Acquire) == 0 {
                assert_eq!(observed_state.data[0].load(Ordering::Relaxed), 1);
                assert_eq!(observed_state.data[1].load(Ordering::Relaxed), 1);
            }
        });
        observer.join().expect("observer must finish");
        for handle in handles {
            handle.join().expect("writer must finish");
        }
    });
}

#[test]
fn test_loom_weak_cas_retry_linearizable() {
    model(|| {
        let counter = Arc::new(AtomicUsize::new(0));

        let c1 = counter.clone();
        let t1 = spawn(move || {
            increment_once(&c1);
        });

        let c2 = counter.clone();
        let t2 = spawn(move || {
            increment_once(&c2);
        });

        t1.join().expect("first increment thread panicked");
        t2.join().expect("second increment thread panicked");
        assert_eq!(counter.load(Ordering::Acquire), 2);
    });
}
