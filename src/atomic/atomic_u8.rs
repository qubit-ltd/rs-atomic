// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Atomic 8-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 8-bit unsigned integer type with sensible
//! default memory orderings.

use std::sync::atomic::AtomicU8 as StdAtomicU8;
use std::sync::atomic::Ordering;

impl_atomic_number!(AtomicU8, StdAtomicU8, u8, "8-bit unsigned integer");
