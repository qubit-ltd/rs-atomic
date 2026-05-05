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
fn test_impl_atomic_integer_value_generated_methods() {
    let atomic = Atomic::<u32>::new(0b0101);

    assert_eq!(atomic.fetch_inc(), 0b0101);
    assert_eq!(atomic.load(), 0b0110);
    assert_eq!(atomic.fetch_dec(), 0b0110);
    assert_eq!(atomic.load(), 0b0101);
    assert_eq!(atomic.fetch_and(0b0011), 0b0101);
    assert_eq!(atomic.load(), 0b0001);
    assert_eq!(atomic.fetch_or(0b1000), 0b0001);
    assert_eq!(atomic.load(), 0b1001);
    assert_eq!(atomic.fetch_xor(0b1010), 0b1001);
    assert_eq!(atomic.load(), 0b0011);
}
