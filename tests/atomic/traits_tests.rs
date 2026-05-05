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
fn test_common_and_numeric_traits_work_together() {
    let atomic = Atomic::<i32>::new(0);

    atomic.store(10);
    assert_eq!(atomic.load(), 10);

    let old = atomic.swap(20);
    assert_eq!(old, 10);
    assert_eq!(atomic.load(), 20);

    assert!(atomic.compare_set(20, 30).is_ok());
    assert_eq!(atomic.load(), 30);

    let prev = atomic.compare_and_exchange(30, 40);
    assert_eq!(prev, 30);
    assert_eq!(atomic.load(), 40);

    assert_eq!(atomic.fetch_add(1), 40);
    assert_eq!(atomic.fetch_sub(1), 41);
    assert_eq!(atomic.load(), 40);
}
