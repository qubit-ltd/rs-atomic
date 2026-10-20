// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Atomic 128-bit Signed Integer
//!
//! Provides an easy-to-use atomic 128-bit signed integer type with sensible
//! default memory orderings.

use std::sync::atomic::Ordering;

use portable_atomic::AtomicI128 as PortableAtomicI128;

impl_atomic_number!(AtomicI128, PortableAtomicI128, i128, "128-bit signed integer");
