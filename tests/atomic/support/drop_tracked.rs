// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};

/// Records how many instances have been dropped during an atomic-reference
/// test.
#[derive(Debug)]
pub(in crate::atomic) struct DropTracked {
    /// Shared counter incremented when this value is dropped.
    pub(in crate::atomic) drops: Arc<AtomicUsize>,
}

impl Drop for DropTracked {
    #[inline]
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
