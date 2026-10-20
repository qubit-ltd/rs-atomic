// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Sealing support for internal marker traits.

/// Prevents downstream crates from implementing the internal atomic value
/// traits.
///
/// Only value types supported by this crate's atomic wrappers implement this
/// trait, keeping the set of atomic backends under the crate's control.
pub trait Sealed {
    // empty
}
