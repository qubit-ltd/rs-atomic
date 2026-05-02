/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Atomic Pointer-Sized Unsigned Integer
//!
//! Provides an easy-to-use atomic pointer-sized unsigned integer type with
//! sensible default memory orderings.
//!

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicUsize,
    std::sync::atomic::AtomicUsize,
    usize,
    "pointer-sized unsigned integer"
);
