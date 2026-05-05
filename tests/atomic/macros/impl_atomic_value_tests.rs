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
fn test_impl_atomic_value_generated_methods() {
    let atomic = Atomic::<bool>::new(false);

    assert!(!atomic.load());
    atomic.store(true);
    assert!(atomic.load());
    assert!(atomic.swap(false));
    assert!(!atomic.load());
}
