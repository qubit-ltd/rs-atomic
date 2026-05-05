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
use std::sync::Arc;

#[test]
fn test_atomic_ops_bool() {
    let atomic = Atomic::<bool>::new(false);

    atomic.store(true);
    assert!(atomic.load());
    let old = atomic.swap(false);
    assert!(old);
    assert!(!atomic.load());
}

#[test]
fn test_atomic_ops_integer() {
    let atomic = Atomic::<i32>::new(0);

    atomic.store(42);
    assert_eq!(atomic.load(), 42);
    let old = atomic.swap(100);
    assert_eq!(old, 42);
    assert_eq!(atomic.load(), 100);

    assert!(atomic.compare_set(100, 200).is_ok());
    assert_eq!(atomic.load(), 200);

    let prev = atomic.compare_and_exchange(200, 300);
    assert_eq!(prev, 200);
    assert_eq!(atomic.load(), 300);
}

#[test]
fn test_atomic_ops_float() {
    let atomic = Atomic::<f32>::new(0.0);

    atomic.store(std::f32::consts::PI);
    assert!((atomic.load() - std::f32::consts::PI).abs() < 1e-6);
    let old = atomic.swap(2.71);
    assert!((old - std::f32::consts::PI).abs() < 1e-6);
}

#[test]
fn test_atomic_ops_ref() {
    let atomic = AtomicRef::new(Arc::new(0));

    atomic.store(Arc::new(42));
    assert_eq!(*atomic.load(), 42);
    let old = atomic.swap(Arc::new(100));
    assert_eq!(*old, 42);
}
