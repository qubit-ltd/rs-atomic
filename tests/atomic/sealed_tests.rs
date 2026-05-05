/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_atomic::{
    Atomic,
    AtomicRef,
};

#[test]
fn test_sealed_atomic_types_are_send_and_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Atomic<bool>>();
    assert_sync::<Atomic<bool>>();

    assert_send::<Atomic<i32>>();
    assert_sync::<Atomic<i32>>();

    assert_send::<Atomic<u64>>();
    assert_sync::<Atomic<u64>>();

    assert_send::<Atomic<f32>>();
    assert_sync::<Atomic<f32>>();

    assert_send::<Atomic<f64>>();
    assert_sync::<Atomic<f64>>();

    assert_send::<AtomicRef<i32>>();
    assert_sync::<AtomicRef<i32>>();
}
