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
fn test_atomic_number_ops_fetch_add_and_sub() {
    let atomic = Atomic::<i32>::new(0);

    assert_eq!(atomic.fetch_add(1), 0);
    assert_eq!(atomic.fetch_add(1), 1);
    assert_eq!(atomic.fetch_sub(1), 2);
    assert_eq!(atomic.fetch_sub(1), 1);
    assert_eq!(atomic.fetch_add(10), 0);
    assert_eq!(atomic.fetch_add(5), 10);
}

#[test]
fn test_atomic_number_ops_combine_with_common_ops() {
    let atomic = Atomic::<i32>::new(0);

    atomic.store(10);
    assert_eq!(atomic.load(), 10);

    let old = atomic.fetch_update(|x| x + 5);
    assert_eq!(old, 10);
    assert_eq!(atomic.load(), 15);

    let old = atomic.fetch_add(1);
    assert_eq!(old, 15);
    assert_eq!(atomic.load(), 16);
}
