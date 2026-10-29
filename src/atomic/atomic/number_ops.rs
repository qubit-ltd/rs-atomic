// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Numeric operations for [`super::Atomic<T>`].

use super::super::atomic_number_ops::AtomicNumberOps;
use super::super::atomic_value::AtomicValue;
use super::Atomic;

impl<T> Atomic<T>
where
    T: AtomicValue,
    T::Primitive: AtomicNumberOps<Value = T>,
{
    /// Adds `delta` to the value and returns the previous value.
    ///
    /// Integer atomics use relaxed ordering for this operation. Floating-point
    /// atomics use a CAS loop. Integer addition wraps on overflow and
    /// underflow.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to add.
    ///
    /// # Returns
    ///
    /// The value before the addition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_add(3), 10);
    /// assert_eq!(atomic.load(), 13);
    /// ```
    #[inline]
    pub fn fetch_add(&self, delta: T) -> T {
        AtomicNumberOps::fetch_add(&self.primitive, delta)
    }

    /// Subtracts `delta` from the value and returns the previous value.
    ///
    /// Integer atomics use relaxed ordering for this operation. Floating-point
    /// atomics use a CAS loop. Integer subtraction wraps on overflow and
    /// underflow.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to subtract.
    ///
    /// # Returns
    ///
    /// The value before the subtraction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_sub(3), 10);
    /// assert_eq!(atomic.load(), 7);
    /// ```
    #[inline]
    pub fn fetch_sub(&self, delta: T) -> T {
        AtomicNumberOps::fetch_sub(&self.primitive, delta)
    }

    /// Multiplies the value by `factor` and returns the previous value.
    ///
    /// This operation uses a CAS loop. Integer multiplication wraps on
    /// overflow and underflow.
    ///
    /// # Parameters
    ///
    /// * `factor` - The value to multiply by.
    ///
    /// # Returns
    ///
    /// The value before the multiplication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.fetch_mul(4), 3);
    /// assert_eq!(atomic.load(), 12);
    /// ```
    #[inline]
    pub fn fetch_mul(&self, factor: T) -> T {
        AtomicNumberOps::fetch_mul(&self.primitive, factor)
    }

    /// Divides the value by `divisor` and returns the previous value.
    ///
    /// This operation uses a CAS loop. Integer division uses wrapping
    /// semantics; for signed integers, `MIN / -1` wraps to `MIN`.
    ///
    /// # Parameters
    ///
    /// * `divisor` - The value to divide by.
    ///
    /// # Returns
    ///
    /// The value before the division.
    ///
    /// # Panics
    ///
    /// For integer specializations, panics if `divisor` is zero. Floating-point
    /// specializations follow IEEE-754 division semantics and do not panic
    /// solely because `divisor` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(20);
    /// assert_eq!(atomic.fetch_div(4), 20);
    /// assert_eq!(atomic.load(), 5);
    /// ```
    #[inline]
    pub fn fetch_div(&self, divisor: T) -> T {
        AtomicNumberOps::fetch_div(&self.primitive, divisor)
    }
}
