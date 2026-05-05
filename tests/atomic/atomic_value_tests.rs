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
fn test_atomic_value_default_trait() {
    let atomic_bool = Atomic::<bool>::default();
    assert!(!atomic_bool.load());

    let atomic_i32 = Atomic::<i32>::default();
    assert_eq!(atomic_i32.load(), 0);

    let atomic_f64 = Atomic::<f64>::default();
    assert_eq!(atomic_f64.load(), 0.0);
}

#[test]
fn test_atomic_value_from_trait() {
    let atomic_bool = Atomic::<bool>::from(true);
    assert!(atomic_bool.load());

    let atomic_i32 = Atomic::<i32>::from(42);
    assert_eq!(atomic_i32.load(), 42);

    let atomic_f32 = Atomic::<f32>::from(std::f32::consts::PI);
    assert!((atomic_f32.load() - std::f32::consts::PI).abs() < 1e-6);
}

#[test]
fn test_atomic_value_debug_display_traits() {
    let atomic_bool = Atomic::<bool>::new(true);
    assert!(format!("{:?}", atomic_bool).contains("true"));
    assert_eq!(format!("{}", atomic_bool), "true");

    let atomic_i32 = Atomic::<i32>::new(42);
    assert!(format!("{:?}", atomic_i32).contains("42"));
    assert_eq!(format!("{}", atomic_i32), "42");

    let atomic_f64 = Atomic::<f64>::new(std::f64::consts::PI);
    assert!(format!("{:?}", atomic_f64).contains("3.14"));
    assert!(format!("{}", atomic_f64).contains("3.14"));
}
