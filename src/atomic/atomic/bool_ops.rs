// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Boolean-specific operations for [`super::Atomic<bool>`].

use super::Atomic;

impl Atomic<bool> {
    /// Stores `true` and returns the previous value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering for this read-modify-write operation.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert_eq!(flag.fetch_set(), false);
    /// assert!(flag.load());
    /// ```
    #[inline]
    pub fn fetch_set(&self) -> bool {
        self.primitive.fetch_set()
    }

    /// Stores `false` and returns the previous value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering for this read-modify-write operation.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert_eq!(flag.fetch_clear(), true);
    /// assert!(!flag.load());
    /// ```
    #[inline]
    pub fn fetch_clear(&self) -> bool {
        self.primitive.fetch_clear()
    }

    /// Negates the value and returns the previous value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering for this read-modify-write operation.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert_eq!(flag.fetch_not(), false);
    /// assert!(flag.load());
    /// ```
    #[inline]
    pub fn fetch_not(&self) -> bool {
        self.primitive.fetch_not()
    }

    /// Applies logical AND and returns the previous value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering for this read-modify-write operation.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to combine with the current value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert_eq!(flag.fetch_and(false), true);
    /// assert!(!flag.load());
    /// ```
    #[inline]
    pub fn fetch_and(&self, value: bool) -> bool {
        self.primitive.fetch_and(value)
    }

    /// Applies logical OR and returns the previous value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering for this read-modify-write operation.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to combine with the current value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert_eq!(flag.fetch_or(true), false);
    /// assert!(flag.load());
    /// ```
    #[inline]
    pub fn fetch_or(&self, value: bool) -> bool {
        self.primitive.fetch_or(value)
    }

    /// Applies logical XOR and returns the previous value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering for this read-modify-write operation.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to combine with the current value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert_eq!(flag.fetch_xor(true), true);
    /// assert!(!flag.load());
    /// ```
    #[inline]
    pub fn fetch_xor(&self, value: bool) -> bool {
        self.primitive.fetch_xor(value)
    }

    /// Stores `new` only when the current value is `false`.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `new` - The replacement value.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(true)` if the observed current value was already `true`.
    /// In that case, `new` is not stored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert!(flag.set_if_false(true).is_ok());
    /// assert!(flag.load());
    /// assert!(flag.set_if_false(false).is_err());
    /// ```
    #[inline]
    pub fn set_if_false(&self, new: bool) -> Result<(), bool> {
        self.primitive.set_if_false(new)
    }

    /// Stores `new` only when the current value is `true`.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `new` - The replacement value.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(false)` if the observed current value was already `false`.
    /// In that case, `new` is not stored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert!(flag.set_if_true(false).is_ok());
    /// assert!(!flag.load());
    /// assert!(flag.set_if_true(true).is_err());
    /// ```
    #[inline]
    pub fn set_if_true(&self, new: bool) -> Result<(), bool> {
        self.primitive.set_if_true(new)
    }
}
