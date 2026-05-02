/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Atomic 64-bit Signed Integer
//!
//! Provides an easy-to-use atomic 64-bit signed integer type with sensible
//! default memory orderings.
//!

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicI64,
    std::sync::atomic::AtomicI64,
    i64,
    "64-bit signed integer"
);
