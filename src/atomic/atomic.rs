// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Generic Atomic Wrapper
//!
//! Provides the public [`Atomic<T>`] wrapper for supported primitive values.

use super::atomic_ops::AtomicOps;
use super::atomic_value::AtomicValue;

// Provides arithmetic methods for supported numeric values.
mod number_ops;
// Provides arithmetic and bitwise methods for integer values.
mod integer_ops;
// Provides logical methods for boolean values.
mod bool_ops;
// Implements standard traits for the atomic wrapper.
mod trait_impls;

/// A high-level atomic wrapper for supported primitive value types.
///
/// This type is the main entry point for primitive atomic values. It hides
/// explicit memory-ordering parameters behind crate-defined defaults while
/// still exposing the raw backend through [`inner`](Self::inner) for advanced
/// use cases.
///
/// # Type Parameters
///
/// * `T` - A supported value type whose `AtomicValue` implementation supplies
///   the atomic backend.
///
/// Supported value types are:
///
/// - `bool`
/// - `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`
/// - `usize`, `isize`
/// - `f32`, `f64`
///
/// The `i128` and `u128` specializations use `portable-atomic` internally
/// because the corresponding standard-library atomic types are not yet stable.
/// Native lock-free support is target-dependent, and the backend fallback may
/// use locks on targets without suitable atomic instructions.
///
/// # Specialization API
///
/// Rustdoc documents [`Atomic<T>`] as one generic type rather than generating a
/// separate page for each concrete `T`. The following table summarizes the
/// methods available for each specialization:
///
/// | Specialization | Additional methods |
/// | --- | --- |
/// | `Atomic<bool>` | `fetch_set`, `fetch_clear`, `fetch_not`, `fetch_and`, `fetch_or`, `fetch_xor`, `set_if_false`, `set_if_true` |
/// | `Atomic<i8>`, `Atomic<i16>`, `Atomic<i32>`, `Atomic<i64>`, `Atomic<i128>`, `Atomic<isize>` | `fetch_add`, `fetch_sub`, `fetch_mul`, `fetch_div`, `fetch_inc`, `fetch_dec`, `fetch_*_with_ordering`, `fetch_and`, `fetch_or`, `fetch_xor`, `fetch_not`, `fetch_accumulate`, `accumulate_and_get`, `fetch_max`, `fetch_min` |
/// | `Atomic<u8>`, `Atomic<u16>`, `Atomic<u32>`, `Atomic<u64>`, `Atomic<u128>`, `Atomic<usize>` | `fetch_add`, `fetch_sub`, `fetch_mul`, `fetch_div`, `fetch_inc`, `fetch_dec`, `fetch_*_with_ordering`, `fetch_and`, `fetch_or`, `fetch_xor`, `fetch_not`, `fetch_accumulate`, `accumulate_and_get`, `fetch_max`, `fetch_min` |
/// | `Atomic<f32>`, `Atomic<f64>` | `fetch_add`, `fetch_sub`, `fetch_mul`, `fetch_div` |
///
/// All supported specializations also provide [`new`](Self::new),
/// [`load`](Self::load), [`store`](Self::store), [`swap`](Self::swap),
/// [`compare_set`](Self::compare_set),
/// [`compare_set_weak`](Self::compare_set_weak),
/// [`compare_and_exchange`](Self::compare_and_exchange),
/// [`fetch_update`](Self::fetch_update),
/// [`update_and_get`](Self::update_and_get),
/// [`try_update`](Self::try_update),
/// [`try_update_and_get`](Self::try_update_and_get), and
/// [`inner`](Self::inner).
///
/// Integer arithmetic operations intentionally follow Rust atomic integer
/// semantics and wrap on overflow and underflow. Use [`crate::AtomicCount`] or
/// [`crate::AtomicSignedCount`] when overflow or underflow must be rejected
/// instead of wrapping.
///
/// Floating-point compare-and-set/exchange operations compare raw
/// [`to_bits`](f32::to_bits) representations, not [`PartialEq`]. This means
/// distinct bit patterns that compare equal, such as `0.0` and `-0.0`, do not
/// match for CAS, and NaN payloads must match exactly. Prefer
/// [`compare_set`](Self::compare_set) or
/// [`compare_set_weak`](Self::compare_set_weak) when the caller needs an
/// explicit success indicator for `f32` or `f64`.
///
/// # Examples
///
/// ```rust
/// use qubit_atomic::Atomic;
///
/// let counter = Atomic::new(0);
/// counter.fetch_inc();
/// assert_eq!(counter.load(), 1);
///
/// let flag = Atomic::new(false);
/// assert_eq!(flag.fetch_set(), false);
/// assert!(flag.load());
/// ```
///
/// When the value type is ambiguous (for example integer literals), specify
/// `T` explicitly with a [turbofish] on the constructor, or by annotating the
/// binding:
///
/// ```rust
/// use qubit_atomic::Atomic;
///
/// let wide: Atomic<u64> = Atomic::new(0);
/// assert_eq!(wide.load(), 0u64);
///
/// let narrow = Atomic::<i16>::new(0);
/// assert_eq!(narrow.load(), 0i16);
/// ```
///
/// [turbofish]: https://doc.rust-lang.org/book/appendix-02-operators.html#the-turbofish
#[doc(alias = "AtomicBool")]
#[doc(alias = "AtomicI8")]
#[doc(alias = "AtomicU8")]
#[doc(alias = "AtomicI16")]
#[doc(alias = "AtomicU16")]
#[doc(alias = "AtomicI32")]
#[doc(alias = "AtomicU32")]
#[doc(alias = "AtomicI64")]
#[doc(alias = "AtomicU64")]
#[doc(alias = "AtomicI128")]
#[doc(alias = "AtomicU128")]
#[doc(alias = "AtomicIsize")]
#[doc(alias = "AtomicUsize")]
#[doc(alias = "AtomicF32")]
#[doc(alias = "AtomicF64")]
#[repr(transparent)]
pub struct Atomic<T>
where
    T: AtomicValue,
{
    /// Primitive backend that performs the concrete atomic operations for `T`.
    primitive: T::Primitive,
}

impl<T> Atomic<T>
where
    T: AtomicValue,
{
    /// Creates a new atomic value.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value.
    ///
    /// # Returns
    ///
    /// An atomic wrapper initialized to `value`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(42);
    /// assert_eq!(atomic.load(), 42);
    /// ```
    ///
    /// To pick a concrete `T` when inference would be ambiguous (for example
    /// `0` as `u64` vs `i32`), write `Atomic::<T>::new(...)` (turbofish) or add
    /// a type annotation on the binding (for example `let x: Atomic<u64> =
    /// ...`):
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let a = Atomic::<u64>::new(0);
    /// assert_eq!(a.load(), 0u64);
    ///
    /// let b: Atomic<isize> = Atomic::new(0);
    /// assert_eq!(b.load(), 0isize);
    /// ```
    #[must_use]
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            primitive: T::new_primitive(value),
        }
    }

    /// Loads the current value.
    ///
    /// Uses `Acquire` ordering by default.
    ///
    /// # Returns
    ///
    /// The current value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(7);
    /// assert_eq!(atomic.load(), 7);
    /// ```
    #[must_use]
    #[inline]
    pub fn load(&self) -> T {
        AtomicOps::load(&self.primitive)
    }

    /// Stores a new value.
    ///
    /// Uses `Release` ordering by default.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to store.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// atomic.store(2);
    /// assert_eq!(atomic.load(), 2);
    /// ```
    #[inline]
    pub fn store(&self, value: T) {
        AtomicOps::store(&self.primitive, value);
    }

    /// Swaps the current value with `value`.
    ///
    /// Unlike [`store`](Self::store), this returns the value that was in the
    /// atomic immediately before the swap, in the same atomic step. Use
    /// [`store`](Self::store) when you do not need the previous value.
    ///
    /// Uses `AcqRel` ordering by default.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to store.
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
    /// let a = Atomic::new(100);
    /// a.store(200);
    /// // store has no return value; you only see the new value via load().
    /// assert_eq!(a.load(), 200);
    ///
    /// let b = Atomic::new(100);
    /// // swap returns the old value and installs the new one atomically.
    /// assert_eq!(b.swap(200), 100);
    /// assert_eq!(b.load(), 200);
    /// ```
    #[must_use]
    #[inline]
    pub fn swap(&self, value: T) -> T {
        AtomicOps::swap(&self.primitive, value)
    }

    /// Compares the current value with `current` and stores `new` on match.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails. In that case, `new` is not stored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// assert!(atomic.compare_set(1, 2).is_ok());
    /// assert_eq!(atomic.load(), 2);
    /// assert_eq!(atomic.compare_set(1, 3), Err(2));
    /// ```
    #[inline]
    pub fn compare_set(&self, current: T, new: T) -> Result<(), T> {
        AtomicOps::compare_set(&self.primitive, current, new)
    }

    /// Weak version of [`compare_set`](Self::compare_set).
    ///
    /// This operation may fail spuriously and is intended for retry loops.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails, including possible spurious failures. In that case, `new` is not
    /// stored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// loop {
    ///     match atomic.compare_set_weak(1, 2) {
    ///         Ok(()) => break,
    ///         Err(actual) => assert_eq!(actual, 1),
    ///     }
    /// }
    /// assert_eq!(atomic.load(), 2);
    /// ```
    #[inline]
    pub fn compare_set_weak(&self, current: T, new: T) -> Result<(), T> {
        AtomicOps::compare_set_weak(&self.primitive, current, new)
    }

    /// Compares and exchanges the value, returning the value seen before the
    /// operation.
    ///
    /// If the return value equals `current`, the exchange succeeded.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// The value observed before the operation completed. If the returned
    /// value equals `current`, the exchange succeeded; otherwise it is the
    /// actual value that prevented the exchange.
    ///
    /// For `Atomic<f32>` and `Atomic<f64>`, CAS compares raw IEEE-754 bit
    /// patterns rather than [`PartialEq`]. A returned floating-point value
    /// comparing equal to `current` is therefore not always enough to prove
    /// success; use [`compare_set`](Self::compare_set) for an explicit
    /// `Ok`/`Err`, or compare [`to_bits`](f32::to_bits) values yourself.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(5);
    /// assert_eq!(atomic.compare_and_exchange(5, 10), 5);
    /// assert_eq!(atomic.load(), 10);
    /// assert_eq!(atomic.compare_and_exchange(5, 0), 10);
    /// ```
    #[must_use]
    #[inline]
    pub fn compare_and_exchange(&self, current: T, new: T) -> T {
        AtomicOps::compare_exchange(&self.primitive, current, new)
    }

    /// Weak version of [`compare_and_exchange`](Self::compare_and_exchange).
    ///
    /// This operation may fail spuriously and is intended for retry loops.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// `Ok(previous)` when the value was replaced, or `Err(actual)` when the
    /// comparison failed, including possible spurious failure.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the exchange fails.
    /// A weak exchange may also return this error spuriously.
    ///
    /// For `Atomic<f32>` and `Atomic<f64>`, the same caveat applies to raw-bit
    /// equality: `0.0` and `-0.0` compare equal by [`PartialEq`] but are
    /// different CAS values. Use [`compare_set_weak`](Self::compare_set_weak)
    /// or compare [`to_bits`](f32::to_bits) values when distinguishing success
    /// from failure matters.
    ///
    /// # Examples
    ///
    /// Weak CAS may fail spuriously; retry on `Err(actual)`.
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(5);
    /// let mut current = 5;
    /// loop {
    ///     match atomic.compare_and_exchange_weak(current, 10) {
    ///         Ok(previous) => {
    ///             assert_eq!(previous, current);
    ///             break;
    ///         }
    ///         Err(actual) => current = actual,
    ///     }
    /// }
    /// assert_eq!(atomic.load(), 10);
    /// ```
    #[inline]
    pub fn compare_and_exchange_weak(&self, current: T, new: T) -> Result<T, T> {
        AtomicOps::compare_exchange_weak(&self.primitive, current, new)
    }

    /// Updates the value with a function and returns the previous value.
    ///
    /// The update uses a CAS loop until it succeeds. The closure may be called
    /// more than once under contention.
    ///
    /// # Type Parameters
    ///
    /// * `F` - An update closure that may be called multiple times during CAS
    ///   retries under contention.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that maps the current value to the next value.
    ///
    /// # Returns
    ///
    /// The value before the successful update.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.fetch_update(|x| x * 2), 3);
    /// assert_eq!(atomic.load(), 6);
    /// ```
    #[inline]
    pub fn fetch_update<F>(&self, f: F) -> T
    where
        F: FnMut(T) -> T,
    {
        AtomicOps::fetch_update(&self.primitive, f)
    }

    /// Updates the value with a function and returns the new value.
    ///
    /// The update uses a CAS loop until it succeeds. The closure may be called
    /// more than once under contention.
    ///
    /// # Type Parameters
    ///
    /// * `F` - An update closure that may be called multiple times during CAS
    ///   retries under contention.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that maps the current value to the next value.
    ///
    /// # Returns
    ///
    /// The value committed by the successful update.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.update_and_get(|x| x * 2), 6);
    /// assert_eq!(atomic.load(), 6);
    /// ```
    #[inline]
    pub fn update_and_get<F>(&self, f: F) -> T
    where
        F: FnMut(T) -> T,
    {
        AtomicOps::update_and_get(&self.primitive, f)
    }

    /// Conditionally updates the value with a function.
    ///
    /// The update uses a CAS loop until it succeeds or the closure rejects the
    /// observed current value by returning `None`. The closure may be called
    /// more than once under contention.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A conditional update closure that may be called multiple times
    ///   during CAS retries under contention.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that maps the current value to `Some(next)` to update
    ///   the atomic, or `None` to leave it unchanged.
    ///
    /// # Returns
    ///
    /// `Some(old_value)` with the value before the successful update, or `None`
    /// when `f` rejects the observed current value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.try_update(|x| (x % 2 == 1).then_some(x + 1)), Some(3));
    /// assert_eq!(atomic.load(), 4);
    /// assert_eq!(atomic.try_update(|x| (x % 2 == 1).then_some(x + 1)), None);
    /// assert_eq!(atomic.load(), 4);
    /// ```
    #[must_use]
    #[inline]
    pub fn try_update<F>(&self, f: F) -> Option<T>
    where
        F: FnMut(T) -> Option<T>,
    {
        AtomicOps::try_update(&self.primitive, f)
    }

    /// Conditionally updates the value with a function and returns the new
    /// value.
    ///
    /// The update uses a CAS loop until it succeeds or the closure rejects the
    /// observed current value by returning `None`. The closure may be called
    /// more than once under contention.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A conditional update closure that may be called multiple times
    ///   during CAS retries under contention.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that maps the current value to `Some(next)` to update
    ///   the atomic, or `None` to leave it unchanged.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` with the value committed by the successful update, or
    /// `None` when `f` rejects the observed current value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(
    ///     atomic.try_update_and_get(|x| (x % 2 == 1).then_some(x + 1)),
    ///     Some(4),
    /// );
    /// assert_eq!(atomic.load(), 4);
    /// assert_eq!(
    ///     atomic.try_update_and_get(|x| (x % 2 == 1).then_some(x + 1)),
    ///     None,
    /// );
    /// assert_eq!(atomic.load(), 4);
    /// ```
    #[must_use]
    #[inline]
    pub fn try_update_and_get<F>(&self, f: F) -> Option<T>
    where
        F: FnMut(T) -> Option<T>,
    {
        AtomicOps::try_update_and_get(&self.primitive, f)
    }

    /// Returns the raw backend atomic value.
    ///
    /// Use this method only when the default orderings are not appropriate
    /// and the caller needs direct access to the backend atomic storage.
    ///
    /// # Returns
    ///
    /// A shared reference to the raw backend atomic value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = Atomic::<i32>::new(0);
    /// assert_eq!(atomic.inner().load(Ordering::Relaxed), 0);
    /// ```
    #[must_use]
    #[inline]
    pub fn inner(&self) -> &T::Inner {
        T::inner(&self.primitive)
    }
}
