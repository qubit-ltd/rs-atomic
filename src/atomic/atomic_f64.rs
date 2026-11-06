// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Atomic 64-bit Floating Point
//!
//! Provides an easy-to-use atomic 64-bit floating point type with sensible
//! default memory orderings. Implemented using bit conversion with AtomicU64.

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crate::atomic::atomic_number_ops::AtomicNumberOps;
use crate::atomic::atomic_ops::AtomicOps;

/// Atomic 64-bit floating point number.
///
/// Provides easy-to-use atomic operations with automatic memory ordering
/// selection. Implemented using `AtomicU64` with bit conversion.
///
/// # Memory Ordering Strategy
///
/// This type uses the same memory ordering strategy as atomic integers:
///
/// - **Read operations** (`load`): Use `Acquire` ordering to ensure visibility
///   of prior writes from other threads.
///
/// - **Write operations** (`store`): Use `Release` ordering to ensure
///   visibility of prior writes to other threads.
///
/// - **Read-Modify-Write operations** (`swap`, `compare_set`): Use `AcqRel`
///   ordering for full synchronization.
///
/// - **CAS-based arithmetic** (`fetch_add`, `fetch_sub`, etc.): Use `AcqRel` on
///   success and `Acquire` on failure within the CAS loop. The loop ensures
///   eventual consistency.
///
/// # Implementation Details
///
/// Since hardware doesn't provide native atomic floating-point operations,
/// this type is implemented using `AtomicU64` with `f64::to_bits()` and
/// `f64::from_bits()` conversions. This preserves bit patterns exactly,
/// including special values like NaN and infinity.
///
/// # Features
///
/// - Automatic memory ordering selection
/// - Arithmetic operations via CAS loops
/// - Inline API over raw-bit atomic storage and CAS loops
/// - Access to underlying type via `inner()` for advanced use cases
///
/// # Limitations
///
/// - Arithmetic operations use CAS loops (slower than integer operations)
/// - CAS comparisons use exact IEEE-754 bit patterns, so different NaN payloads
///   and `0.0`/`-0.0` are treated as different values
/// - No max/min operations (complex floating point semantics)
///
/// # Examples
///
/// ```rust
/// use qubit_atomic::atomic::primitive::AtomicF64;
///
/// let atomic = AtomicF64::new(3.14159);
/// atomic.fetch_add(1.0);
/// assert_eq!(atomic.load(), 4.14159);
/// ```
#[repr(transparent)]
pub struct AtomicF64 {
    /// Raw-bit atomic storage for the `f64` value.
    inner: AtomicU64,
}

impl AtomicF64 {
    /// Creates a new atomic floating point number.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value.
    ///
    /// # Returns
    ///
    /// An atomic `f64` initialized to `value`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<f64>::new(3.14159);
    /// assert_eq!(atomic.load(), 3.14159);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self {
            inner: AtomicU64::new(value.to_bits()),
        }
    }

    /// Gets the current value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Acquire` ordering on the underlying `AtomicU64`. This ensures
    /// that all writes from other threads that happened before a `Release`
    /// store are visible after this load.
    ///
    /// # Returns
    ///
    /// The current value.
    #[must_use]
    #[inline]
    pub fn load(&self) -> f64 {
        f64::from_bits(self.inner.load(Ordering::Acquire))
    }

    /// Gets a reference to the underlying standard library atomic type.
    ///
    /// This allows direct access to the standard library's atomic operations
    /// for advanced use cases that require fine-grained control over memory
    /// ordering.
    ///
    /// # Memory Ordering
    ///
    /// When using the returned reference, you have full control over memory
    /// ordering. Remember to use `f64::to_bits()` and `f64::from_bits()` for
    /// conversions.
    ///
    /// # Returns
    ///
    /// A reference to the underlying `std::sync::atomic::AtomicU64`.
    #[must_use]
    #[inline]
    pub fn inner(&self) -> &AtomicU64 {
        &self.inner
    }

    /// Sets a new value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Release` ordering on the underlying `AtomicU64`. This ensures
    /// that all prior writes in this thread are visible to other threads
    /// that perform an `Acquire` load.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to set.
    #[inline]
    pub fn store(&self, value: f64) {
        self.inner.store(value.to_bits(), Ordering::Release);
    }

    /// Swaps the current value with a new value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering on the underlying `AtomicU64`. This provides
    /// full synchronization for this read-modify-write operation.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to swap in.
    ///
    /// # Returns
    ///
    /// The old value.
    #[must_use]
    #[inline]
    pub fn swap(&self, value: f64) -> f64 {
        f64::from_bits(self.inner.swap(value.to_bits(), Ordering::AcqRel))
    }

    /// Compares and sets the value atomically.
    ///
    /// If the current value equals `current`, sets it to `new` and returns
    /// `Ok(())`. Otherwise, returns `Err(actual)` where `actual` is the
    /// current value.
    ///
    /// Comparison uses the exact raw bit pattern produced by
    /// [`f64::to_bits`], not [`PartialEq`].
    ///
    /// # Memory Ordering
    ///
    /// - **Success**: Uses `AcqRel` ordering on the underlying `AtomicU64` to
    ///   ensure full synchronization when the exchange succeeds.
    /// - **Failure**: Uses `Acquire` ordering to observe the actual value
    ///   written by another thread.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the raw-bit
    /// comparison fails. In that case, `new` is not stored.
    #[inline]
    pub fn compare_set(&self, current: f64, new: f64) -> Result<(), f64> {
        self.inner
            .compare_exchange(current.to_bits(), new.to_bits(), Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(f64::from_bits)
    }

    /// Weak version of compare-and-set.
    ///
    /// May spuriously fail even when the comparison succeeds. Should be used
    /// in a loop.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    /// Comparison uses the exact raw bit pattern produced by
    /// [`f64::to_bits`].
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the raw-bit
    /// comparison fails, including possible spurious failures. In that case,
    /// `new` is not stored.
    #[inline]
    pub fn compare_set_weak(&self, current: f64, new: f64) -> Result<(), f64> {
        self.inner
            .compare_exchange_weak(current.to_bits(), new.to_bits(), Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(f64::from_bits)
    }

    /// Compares and exchanges the value atomically, returning the previous
    /// value.
    ///
    /// If the current value equals `current`, sets it to `new` and returns
    /// the old value. Otherwise, returns the actual current value.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// The value observed before the operation completed. If the returned
    /// value has the same raw bits as `current`, the exchange succeeded;
    /// otherwise it is the actual value that prevented the exchange.
    #[must_use]
    #[inline]
    pub fn compare_and_exchange(&self, current: f64, new: f64) -> f64 {
        match self
            .inner
            .compare_exchange(current.to_bits(), new.to_bits(), Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(prev_bits) => f64::from_bits(prev_bits),
            Err(actual_bits) => f64::from_bits(actual_bits),
        }
    }

    /// Weak version of compare-and-exchange.
    ///
    /// May spuriously fail even when the comparison succeeds. Should be used
    /// in a loop.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(previous)` when the value was replaced, or `Err(actual)` when the
    /// comparison failed, including possible spurious failure. Values preserve
    /// their exact raw bit patterns.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` when the raw-bit comparison fails or the operation
    /// fails spuriously. In that case, `new` is not stored.
    #[inline]
    pub fn compare_and_exchange_weak(&self, current: f64, new: f64) -> Result<f64, f64> {
        self.inner
            .compare_exchange_weak(current.to_bits(), new.to_bits(), Ordering::AcqRel, Ordering::Acquire)
            .map(f64::from_bits)
            .map_err(f64::from_bits)
    }

    /// Atomically adds a value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Performance
    ///
    /// May be slow in high-contention scenarios due to the CAS loop.
    /// Consider using atomic integers if performance is critical.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to add.
    ///
    /// # Returns
    ///
    /// The old value before adding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<f64>::new(10.0);
    /// let old = atomic.fetch_add(5.5);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 15.5);
    /// ```
    #[inline]
    pub fn fetch_add(&self, delta: f64) -> f64 {
        self.fetch_update(|current| current + delta)
    }

    /// Atomically subtracts a value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to subtract.
    ///
    /// # Returns
    ///
    /// The old value before subtracting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<f64>::new(10.0);
    /// let old = atomic.fetch_sub(3.5);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 6.5);
    /// ```
    #[inline]
    pub fn fetch_sub(&self, delta: f64) -> f64 {
        self.fetch_update(|current| current - delta)
    }

    /// Atomically multiplies by a factor, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Parameters
    ///
    /// * `factor` - The factor to multiply by.
    ///
    /// # Returns
    ///
    /// The old value before multiplying.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<f64>::new(10.0);
    /// let old = atomic.fetch_mul(2.5);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 25.0);
    /// ```
    #[inline]
    pub fn fetch_mul(&self, factor: f64) -> f64 {
        self.fetch_update(|current| current * factor)
    }

    /// Atomically divides by a divisor, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Parameters
    ///
    /// * `divisor` - The divisor to divide by.
    ///
    /// # Returns
    ///
    /// The old value before dividing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<f64>::new(10.0);
    /// let old = atomic.fetch_div(2.0);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 5.0);
    /// ```
    #[inline]
    pub fn fetch_div(&self, divisor: f64) -> f64 {
        self.fetch_update(|current| current / divisor)
    }

    /// Updates the value using a function, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Type Parameters
    ///
    /// * `F` - An update closure that may run more than once during CAS
    ///   retries.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current value and returns the new
    ///   value.
    ///
    /// # Returns
    ///
    /// The old value before the update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    pub fn fetch_update<F>(&self, mut f: F) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        let mut current = self.load();
        loop {
            let new = f(current);
            match self.compare_set_weak(current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Updates the value using a function, returning the new value.
    ///
    /// Internally uses a CAS loop until the update succeeds.
    ///
    /// # Type Parameters
    ///
    /// * `F` - An update closure that may run more than once during CAS
    ///   retries.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current value and returns the new
    ///   value.
    ///
    /// # Returns
    ///
    /// The value committed by the successful update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    pub fn update_and_get<F>(&self, mut f: F) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        let mut current = self.load();
        loop {
            let new = f(current);
            match self.compare_set_weak(current, new) {
                Ok(_) => return new,
                Err(actual) => current = actual,
            }
        }
    }

    /// Conditionally updates the value using a function.
    ///
    /// Internally uses a CAS loop until the update succeeds or the closure
    /// rejects the current value by returning `None`.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A conditional update closure that may run more than once during
    ///   CAS retries.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current value and returns the new
    ///   value, or `None` to leave the value unchanged.
    ///
    /// # Returns
    ///
    /// `Some(old_value)` when the update succeeds, or `None` when `f` rejects
    /// the observed current value.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    #[must_use]
    pub fn try_update<F>(&self, mut f: F) -> Option<f64>
    where
        F: FnMut(f64) -> Option<f64>,
    {
        let mut current = self.load();
        loop {
            let new = f(current)?;
            match self.compare_set_weak(current, new) {
                Ok(_) => return Some(current),
                Err(actual) => current = actual,
            }
        }
    }

    /// Conditionally updates the value using a function, returning the new
    /// value.
    ///
    /// Internally uses a CAS loop until the update succeeds or the closure
    /// rejects the current value by returning `None`.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A conditional update closure that may run more than once during
    ///   CAS retries.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current value and returns the new
    ///   value, or `None` to leave the value unchanged.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` when the update succeeds, or `None` when `f` rejects
    /// the observed current value.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    #[must_use]
    pub fn try_update_and_get<F>(&self, mut f: F) -> Option<f64>
    where
        F: FnMut(f64) -> Option<f64>,
    {
        let mut current = self.load();
        loop {
            let new = f(current)?;
            match self.compare_set_weak(current, new) {
                Ok(_) => return Some(new),
                Err(actual) => current = actual,
            }
        }
    }
}

impl AtomicOps for AtomicF64 {
    /// Floating-point value type stored by this atomic backend.
    type Value = f64;

    /// Loads the current value using acquire ordering.
    ///
    /// # Returns
    ///
    /// The value currently stored in the atomic.
    #[inline]
    fn load(&self) -> f64 {
        self.load()
    }

    /// Stores a value using release ordering.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to store.
    #[inline]
    fn store(&self, value: f64) {
        self.store(value);
    }

    /// Replaces the current value and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to store.
    ///
    /// # Returns
    ///
    /// The value observed before replacement.
    #[inline]
    fn swap(&self, value: f64) -> f64 {
        self.swap(value)
    }

    /// Replaces the value only when its raw bits match `current`.
    ///
    /// Returns `Err(actual)` if the comparison fails; floating-point equality
    /// follows the exact bit representation rather than `PartialEq`.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected value, compared by its raw bits.
    /// * `new` - The value to store if the comparison succeeds.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(actual)` with the observed value on
    /// failure.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` when the raw-bit comparison fails.
    #[inline]
    fn compare_set(&self, current: f64, new: f64) -> Result<(), f64> {
        self.compare_set(current, new)
    }

    /// Weakly replaces the value when its raw bits match `current`.
    ///
    /// The operation may fail spuriously, so callers generally retry it in a
    /// loop. Returns the observed value on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected value, compared by its raw bits.
    /// * `new` - The value to store if the comparison succeeds.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(actual)` with the observed value on
    /// failure.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` when the comparison fails or the operation fails
    /// spuriously.
    #[inline]
    fn compare_set_weak(&self, current: f64, new: f64) -> Result<(), f64> {
        self.compare_set_weak(current, new)
    }

    /// Compares raw bits and returns the value observed by the exchange.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected value, compared by its raw bits.
    /// * `new` - The value to store if the comparison succeeds.
    ///
    /// # Returns
    ///
    /// The value observed before the operation completed.
    #[inline]
    fn compare_exchange(&self, current: f64, new: f64) -> f64 {
        self.compare_and_exchange(current, new)
    }

    /// Weakly compares raw bits and returns whether the exchange succeeded.
    ///
    /// The operation may fail spuriously; the result preserves the observed
    /// floating-point bit pattern.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected value, compared by its raw bits.
    /// * `new` - The value to store if the comparison succeeds.
    ///
    /// # Returns
    ///
    /// `Ok(previous)` on success, or `Err(actual)` on failure.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` if the comparison fails or the operation fails
    /// spuriously.
    #[inline]
    fn compare_exchange_weak(&self, current: f64, new: f64) -> Result<f64, f64> {
        self.compare_and_exchange_weak(current, new)
    }

    /// Applies an update closure atomically and returns the previous value.
    ///
    /// The closure may run multiple times when concurrent updates cause
    /// compare-and-set retries.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The closure type used to calculate a candidate value.
    ///
    /// # Parameters
    ///
    /// * `f` - A closure that receives the current value and returns a
    ///   replacement.
    ///
    /// # Returns
    ///
    /// The value observed before the successful update.
    #[inline]
    fn fetch_update<F>(&self, f: F) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        self.fetch_update(f)
    }

    /// Applies an update closure atomically and returns the committed value.
    ///
    /// The closure may run multiple times when concurrent updates cause
    /// compare-and-set retries.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The closure type used to calculate a candidate value.
    ///
    /// # Parameters
    ///
    /// * `f` - A closure that receives the current value and returns a
    ///   replacement.
    ///
    /// # Returns
    ///
    /// The value committed by the successful update.
    #[inline]
    fn update_and_get<F>(&self, f: F) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        self.update_and_get(f)
    }

    /// Applies an update unless the closure rejects the observed value.
    ///
    /// Returns the previous value on success and `None` on rejection. The
    /// closure may run multiple times during compare-and-set retries.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The closure type used to accept or reject a candidate value.
    ///
    /// # Parameters
    ///
    /// * `f` - A closure that returns a replacement or `None` to reject it.
    ///
    /// # Returns
    ///
    /// `Some(previous)` on success, or `None` when the closure rejects a value.
    #[inline]
    fn try_update<F>(&self, f: F) -> Option<f64>
    where
        F: FnMut(f64) -> Option<f64>,
    {
        self.try_update(f)
    }

    /// Applies an update unless the closure rejects the observed value.
    ///
    /// Returns the committed value on success and `None` on rejection. The
    /// closure may run multiple times during compare-and-set retries.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The closure type used to accept or reject a candidate value.
    ///
    /// # Parameters
    ///
    /// * `f` - A closure that returns a replacement or `None` to reject it.
    ///
    /// # Returns
    ///
    /// `Some(committed)` on success, or `None` when the closure rejects a
    /// value.
    #[inline]
    fn try_update_and_get<F>(&self, f: F) -> Option<f64>
    where
        F: FnMut(f64) -> Option<f64>,
    {
        self.try_update_and_get(f)
    }
}

impl AtomicNumberOps for AtomicF64 {
    /// Adds `delta` atomically and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to add.
    ///
    /// # Returns
    ///
    /// The value observed before addition.
    #[inline]
    fn fetch_add(&self, delta: f64) -> f64 {
        self.fetch_add(delta)
    }

    /// Subtracts `delta` atomically and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to subtract.
    ///
    /// # Returns
    ///
    /// The value observed before subtraction.
    #[inline]
    fn fetch_sub(&self, delta: f64) -> f64 {
        self.fetch_sub(delta)
    }

    /// Multiplies the value by `factor` atomically and returns the previous
    /// value.
    ///
    /// # Parameters
    ///
    /// * `factor` - The multiplier.
    ///
    /// # Returns
    ///
    /// The value observed before multiplication.
    #[inline]
    fn fetch_mul(&self, factor: f64) -> f64 {
        self.fetch_mul(factor)
    }

    /// Divides the value by `divisor` atomically and returns the previous
    /// value.
    ///
    /// Division follows IEEE-754 semantics, including division by zero.
    ///
    /// # Parameters
    ///
    /// * `divisor` - The divisor.
    ///
    /// # Returns
    ///
    /// The value observed before division.
    #[inline]
    fn fetch_div(&self, divisor: f64) -> f64 {
        self.fetch_div(divisor)
    }
}
