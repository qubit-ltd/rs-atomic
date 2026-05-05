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

#[test]
fn test_atomic_integer_value_i8_operations() {
    let atomic = Atomic::<i8>::new(0);

    assert_eq!(atomic.fetch_inc(), 0);
    assert_eq!(atomic.load(), 1);
    assert_eq!(atomic.fetch_add(5), 1);
    assert_eq!(atomic.load(), 6);
    assert_eq!(atomic.fetch_dec(), 6);
    assert_eq!(atomic.load(), 5);

    atomic.store(0b0101);
    assert_eq!(atomic.fetch_and(0b0011), 0b0101);
    assert_eq!(atomic.load(), 0b0001);
}

#[test]
fn test_atomic_integer_value_u16_min_max() {
    let atomic = Atomic::<u16>::new(0);

    assert_eq!(atomic.fetch_inc(), 0);
    assert_eq!(atomic.fetch_add(10), 1);
    assert_eq!(atomic.fetch_sub(5), 11);
    assert_eq!(atomic.load(), 6);

    assert_eq!(atomic.fetch_max(20), 6);
    assert_eq!(atomic.load(), 20);

    assert_eq!(atomic.fetch_min(10), 20);
    assert_eq!(atomic.load(), 10);
}

#[test]
fn test_atomic_integer_value_accumulate_and_bitwise() {
    let atomic = Atomic::<i32>::new(0);

    let old = atomic.fetch_accumulate(5, |a, b| a + b);
    assert_eq!(old, 0);
    assert_eq!(atomic.load(), 5);

    let old = atomic.fetch_accumulate(10, |a, b| a * b);
    assert_eq!(old, 5);
    assert_eq!(atomic.load(), 50);

    let atomic = Atomic::<i64>::new(0);
    atomic.fetch_inc();
    atomic.fetch_add(99);
    assert_eq!(atomic.load(), 100);

    atomic.fetch_or(0b1111);
    assert_eq!(atomic.load() & 0b1111, 0b1111);
}

#[test]
fn test_atomic_integer_value_usize_operations() {
    let atomic = Atomic::<usize>::new(0);

    for _ in 0..10 {
        atomic.fetch_inc();
    }
    assert_eq!(atomic.load(), 10);

    atomic.fetch_xor(0b1010);
    let _ = atomic.load();
}
