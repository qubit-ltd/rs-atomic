/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_atomic::Atomic;
use qubit_atomic::atomic::primitive::AtomicI64;
use std::sync::atomic::Ordering;

#[test]
fn test_impl_atomic_number_generated_methods() {
    let atomic = Atomic::<i64>::new(10);

    assert_eq!(atomic.fetch_add(5), 10);
    assert_eq!(atomic.load(), 15);
    assert_eq!(atomic.fetch_sub(3), 15);
    assert_eq!(atomic.load(), 12);
    assert_eq!(atomic.fetch_mul(2), 12);
    assert_eq!(atomic.load(), 24);
    assert_eq!(atomic.fetch_div(4), 24);
    assert_eq!(atomic.load(), 6);
    assert_eq!(atomic.accumulate_and_get(4, |a, b| a + b), 10);
    assert_eq!(atomic.load(), 10);
}

#[test]
fn test_impl_atomic_number_generated_ordered_methods() {
    let atomic = AtomicI64::new(10);

    assert_eq!(atomic.fetch_add_with_ordering(5, Ordering::AcqRel), 10);
    assert_eq!(atomic.load(), 15);
    assert_eq!(atomic.fetch_sub_with_ordering(3, Ordering::AcqRel), 15);
    assert_eq!(atomic.load(), 12);
    assert_eq!(atomic.fetch_inc_with_ordering(Ordering::AcqRel), 12);
    assert_eq!(atomic.load(), 13);
    assert_eq!(atomic.fetch_dec_with_ordering(Ordering::AcqRel), 13);
    assert_eq!(atomic.load(), 12);
}
