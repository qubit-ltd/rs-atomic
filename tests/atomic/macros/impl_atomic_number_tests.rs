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
}
