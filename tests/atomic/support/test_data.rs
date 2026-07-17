// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

/// Structured value used by atomic-reference tests.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::atomic) struct TestData {
    /// Numeric payload used to verify reference updates.
    pub(in crate::atomic) value: i32,
    /// Text payload used to verify whole-value replacement.
    pub(in crate::atomic) name: String,
}
