/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! # Atomic 16-bit Signed Integer
//!
//! Provides an easy-to-use atomic 16-bit signed integer type with sensible
//! default memory orderings.
//!

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicI16,
    std::sync::atomic::AtomicI16,
    i16,
    "16-bit signed integer"
);
