// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use loom::sync::Arc;
use loom::sync::atomic::{
    AtomicBool,
    AtomicUsize,
    Ordering,
};
use loom::thread;
use qubit_atomic::atomic::testing::try_update_atomic_count;

/// Performs one checked increment through the production `AtomicCount` core.
fn increment_once(counter: &AtomicUsize) {
    assert!(
        try_update_atomic_count(
            || counter.load(Ordering::Acquire),
            |current, next| {
                counter.compare_exchange_weak(
                    current,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            },
            |current| current.checked_add(1),
        )
        .is_some()
    );
}

#[test]
fn test_loom_release_acquire_visibility() {
    loom::model(|| {
        let data = Arc::new(AtomicUsize::new(0));
        let ready = Arc::new(AtomicBool::new(false));

        let data_writer = data.clone();
        let ready_writer = ready.clone();
        let writer = thread::spawn(move || {
            // Publish data first, then publish the ready flag.
            data_writer.store(42, Ordering::Relaxed);
            ready_writer.store(true, Ordering::Release);
        });

        let data_reader = data.clone();
        let ready_reader = ready.clone();
        let reader = thread::spawn(move || {
            while !ready_reader.load(Ordering::Acquire) {
                thread::yield_now();
            }
            assert_eq!(data_reader.load(Ordering::Relaxed), 42);
        });

        writer.join().expect("writer thread panicked");
        reader.join().expect("reader thread panicked");
    });
}

#[test]
fn test_loom_checked_update_prevents_underflow() {
    loom::model(|| {
        let counter = Arc::new(AtomicUsize::new(1));

        let c1 = counter.clone();
        let t1 = thread::spawn(move || {
            try_update_atomic_count(
                || c1.load(Ordering::Acquire),
                |current, next| {
                    c1.compare_exchange_weak(
                        current,
                        next,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                },
                |current| current.checked_sub(1),
            )
        });

        let c2 = counter.clone();
        let t2 = thread::spawn(move || {
            try_update_atomic_count(
                || c2.load(Ordering::Acquire),
                |current, next| {
                    c2.compare_exchange_weak(
                        current,
                        next,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                },
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
fn test_loom_weak_cas_retry_linearizable() {
    loom::model(|| {
        let counter = Arc::new(AtomicUsize::new(0));

        let c1 = counter.clone();
        let t1 = thread::spawn(move || {
            increment_once(&c1);
        });

        let c2 = counter.clone();
        let t2 = thread::spawn(move || {
            increment_once(&c2);
        });

        t1.join().expect("first increment thread panicked");
        t2.join().expect("second increment thread panicked");
        assert_eq!(counter.load(Ordering::Acquire), 2);
    });
}
