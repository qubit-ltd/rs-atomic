// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Atomic Performance Benchmarks
//!
//! Criterion comparisons between `qubit-atomic` wrappers and their direct
//! standard-library or `arc-swap` equivalents.

use arc_swap::ArcSwap;
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use qubit_atomic::{
    Atomic,
    AtomicCount,
    AtomicRef,
};
use std::hint::black_box;
use std::sync::Arc;
use std::sync::atomic::{
    AtomicI32,
    AtomicUsize,
    Ordering,
};

/// Benchmarks acquire loads against the equivalent standard-library atomic.
fn benchmark_primitive_load(c: &mut Criterion) {
    let wrapper = Atomic::<i32>::new(42);
    let direct = AtomicI32::new(42);
    let mut group = c.benchmark_group("primitive_load_acquire");

    group.bench_function("qubit_atomic", |b| {
        b.iter(|| black_box(wrapper.load()));
    });
    group.bench_function("std", |b| {
        b.iter(|| black_box(direct.load(Ordering::Acquire)));
    });
    group.finish();
}

/// Benchmarks relaxed increments against the equivalent standard-library RMW.
fn benchmark_primitive_increment(c: &mut Criterion) {
    let wrapper = Atomic::<i32>::new(0);
    let direct = AtomicI32::new(0);
    let mut group = c.benchmark_group("primitive_increment_relaxed");

    group.bench_function("qubit_atomic", |b| {
        b.iter(|| black_box(wrapper.fetch_inc()));
    });
    group.bench_function("std", |b| {
        b.iter(|| black_box(direct.fetch_add(1, Ordering::Relaxed)));
    });
    group.finish();
}

/// Benchmarks checked count increments against an equivalent standard CAS loop.
fn benchmark_checked_count_increment(c: &mut Criterion) {
    let wrapper = AtomicCount::zero();
    let direct = AtomicUsize::new(0);
    let mut group = c.benchmark_group("checked_count_increment");

    group.bench_function("qubit_atomic", |b| {
        b.iter(|| black_box(wrapper.inc()));
    });
    group.bench_function("std_fetch_update", |b| {
        b.iter(|| black_box(checked_increment(&direct)));
    });
    group.finish();
}

/// Benchmarks owned reference loads against direct `ArcSwap::load_full` calls.
fn benchmark_reference_load(c: &mut Criterion) {
    let value = Arc::new(String::from("configuration"));
    let wrapper = AtomicRef::new(value.clone());
    let direct = ArcSwap::from(value);
    let mut group = c.benchmark_group("reference_owned_load");

    group.bench_function("qubit_atomic", |b| {
        b.iter(|| black_box(wrapper.load()));
    });
    group.bench_function("arc_swap", |b| {
        b.iter(|| black_box(direct.load_full()));
    });
    group.finish();
}

/// Increments `counter` without wrapping and returns the committed new value.
fn checked_increment(counter: &AtomicUsize) -> usize {
    let mut current = counter.load(Ordering::Acquire);
    loop {
        let next = current.checked_add(1).expect("benchmark counter overflow");
        match counter.compare_exchange_weak(
            current,
            next,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return next,
            Err(actual) => current = actual,
        }
    }
}

criterion_group!(
    benches,
    benchmark_primitive_load,
    benchmark_primitive_increment,
    benchmark_checked_count_increment,
    benchmark_reference_load
);
criterion_main!(benches);
