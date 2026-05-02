/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Atomic 16-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 16-bit unsigned integer type with sensible
//! default memory orderings.
//!

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicU16,
    std::sync::atomic::AtomicU16,
    u16,
    "16-bit unsigned integer"
);
