// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Atomic 128-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 128-bit unsigned integer type with sensible
//! default memory orderings.

use std::sync::atomic::Ordering;

use portable_atomic::AtomicU128 as PortableAtomicU128;

impl_atomic_number!(AtomicU128, PortableAtomicU128, u128, "128-bit unsigned integer");
