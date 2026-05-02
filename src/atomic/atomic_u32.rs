/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Atomic 32-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 32-bit unsigned integer type with sensible
//! default memory orderings.
//!

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicU32,
    std::sync::atomic::AtomicU32,
    u32,
    "32-bit unsigned integer"
);
