// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integer-only operations for [`super::Atomic<T>`].

use std::sync::atomic::Ordering;

use super::super::atomic_integer_value::AtomicIntegerValue;
use super::Atomic;

impl<T> Atomic<T>
where
    T: AtomicIntegerValue,
{
    /// Increments the value by one and returns the previous value.
    ///
    /// # Returns
    ///
    /// The value before the increment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(0);
    /// assert_eq!(atomic.fetch_inc(), 0);
    /// assert_eq!(atomic.load(), 1);
    /// ```
    #[inline]
    pub fn fetch_inc(&self) -> T {
        T::fetch_inc(&self.primitive)
    }

    /// Increments the value by one with an explicit memory ordering and returns
    /// the previous value.
    ///
    /// Arithmetic wraps on overflow, matching Rust atomic integer operations.
    ///
    /// # Parameters
    ///
    /// * `ordering` - The memory ordering used by the atomic read-modify-write
    ///   operation.
    ///
    /// # Returns
    ///
    /// The value before the increment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = Atomic::new(0);
    /// assert_eq!(atomic.fetch_inc_with_ordering(Ordering::AcqRel), 0);
    /// assert_eq!(atomic.load(), 1);
    /// ```
    #[inline]
    pub fn fetch_inc_with_ordering(&self, ordering: Ordering) -> T {
        T::fetch_inc_with_ordering(&self.primitive, ordering)
    }

    /// Decrements the value by one and returns the previous value.
    ///
    /// # Returns
    ///
    /// The value before the decrement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// assert_eq!(atomic.fetch_dec(), 1);
    /// assert_eq!(atomic.load(), 0);
    /// ```
    #[inline]
    pub fn fetch_dec(&self) -> T {
        T::fetch_dec(&self.primitive)
    }

    /// Decrements the value by one with an explicit memory ordering and returns
    /// the previous value.
    ///
    /// Arithmetic wraps on underflow, matching Rust atomic integer operations.
    ///
    /// # Parameters
    ///
    /// * `ordering` - The memory ordering used by the atomic read-modify-write
    ///   operation.
    ///
    /// # Returns
    ///
    /// The value before the decrement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = Atomic::new(1);
    /// assert_eq!(atomic.fetch_dec_with_ordering(Ordering::AcqRel), 1);
    /// assert_eq!(atomic.load(), 0);
    /// ```
    #[inline]
    pub fn fetch_dec_with_ordering(&self, ordering: Ordering) -> T {
        T::fetch_dec_with_ordering(&self.primitive, ordering)
    }

    /// Adds `delta` with an explicit memory ordering and returns the previous
    /// value.
    ///
    /// Arithmetic wraps on overflow and underflow, matching Rust atomic integer
    /// operations.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to add.
    /// * `ordering` - The memory ordering used by the atomic read-modify-write
    ///   operation.
    ///
    /// # Returns
    ///
    /// The value before the addition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_add_with_ordering(5, Ordering::AcqRel), 10);
    /// assert_eq!(atomic.load(), 15);
    /// ```
    #[inline]
    pub fn fetch_add_with_ordering(&self, delta: T, ordering: Ordering) -> T {
        T::fetch_add_with_ordering(&self.primitive, delta, ordering)
    }

    /// Subtracts `delta` with an explicit memory ordering and returns the
    /// previous value.
    ///
    /// Arithmetic wraps on overflow and underflow, matching Rust atomic integer
    /// operations.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to subtract.
    /// * `ordering` - The memory ordering used by the atomic read-modify-write
    ///   operation.
    ///
    /// # Returns
    ///
    /// The value before the subtraction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_sub_with_ordering(3, Ordering::AcqRel), 10);
    /// assert_eq!(atomic.load(), 7);
    /// ```
    #[inline]
    pub fn fetch_sub_with_ordering(&self, delta: T, ordering: Ordering) -> T {
        T::fetch_sub_with_ordering(&self.primitive, delta, ordering)
    }

    /// Applies bitwise AND and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The mask to apply.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<u8>::new(0b1111);
    /// assert_eq!(atomic.fetch_and(0b1010), 0b1111);
    /// assert_eq!(atomic.load(), 0b1010);
    /// ```
    #[inline]
    pub fn fetch_and(&self, value: T) -> T {
        T::fetch_and(&self.primitive, value)
    }

    /// Applies bitwise OR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The mask to apply.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<u8>::new(0b1000);
    /// assert_eq!(atomic.fetch_or(0b0011), 0b1000);
    /// assert_eq!(atomic.load(), 0b1011);
    /// ```
    #[inline]
    pub fn fetch_or(&self, value: T) -> T {
        T::fetch_or(&self.primitive, value)
    }

    /// Applies bitwise XOR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The mask to apply.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<u8>::new(0b1111);
    /// assert_eq!(atomic.fetch_xor(0b1010), 0b1111);
    /// assert_eq!(atomic.load(), 0b0101);
    /// ```
    #[inline]
    pub fn fetch_xor(&self, value: T) -> T {
        T::fetch_xor(&self.primitive, value)
    }

    /// Flips all bits and returns the previous value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<i32>::new(0);
    /// assert_eq!(atomic.fetch_not(), 0);
    /// assert_eq!(atomic.load(), !0);
    /// ```
    #[inline]
    pub fn fetch_not(&self) -> T {
        T::fetch_not(&self.primitive)
    }

    /// Updates the value by accumulating it with `value`.
    ///
    /// # Type Parameters
    ///
    /// * `F` - An accumulator closure that may be called multiple times during
    ///   CAS retries under contention.
    ///
    /// # Parameters
    ///
    /// * `value` - The right-hand input to the accumulator.
    /// * `f` - A function that combines the current value and `value`.
    ///
    /// # Returns
    ///
    /// The value before the successful update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_accumulate(5, |a, b| a + b), 10);
    /// assert_eq!(atomic.load(), 15);
    /// ```
    #[inline]
    pub fn fetch_accumulate<F>(&self, value: T, f: F) -> T
    where
        F: FnMut(T, T) -> T,
    {
        T::fetch_accumulate(&self.primitive, value, f)
    }

    /// Updates the value by accumulating it with `value` and returns the new
    /// value.
    ///
    /// # Type Parameters
    ///
    /// * `F` - An accumulator closure that may be called multiple times during
    ///   CAS retries under contention.
    ///
    /// # Parameters
    ///
    /// * `value` - The right-hand input to the accumulator.
    /// * `f` - A function that combines the current value and `value`.
    ///
    /// # Returns
    ///
    /// The value committed by the successful update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.accumulate_and_get(5, |a, b| a + b), 15);
    /// assert_eq!(atomic.load(), 15);
    /// ```
    #[inline]
    pub fn accumulate_and_get<F>(&self, value: T, f: F) -> T
    where
        F: FnMut(T, T) -> T,
    {
        T::accumulate_and_get(&self.primitive, value, f)
    }

    /// Replaces the value with the maximum of the current value and `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to compare with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.fetch_max(10), 3);
    /// assert_eq!(atomic.load(), 10);
    /// ```
    #[inline]
    pub fn fetch_max(&self, value: T) -> T {
        T::fetch_max(&self.primitive, value)
    }

    /// Replaces the value with the minimum of the current value and `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to compare with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_min(3), 10);
    /// assert_eq!(atomic.load(), 3);
    /// ```
    #[inline]
    pub fn fetch_min(&self, value: T) -> T {
        T::fetch_min(&self.primitive, value)
    }
}
