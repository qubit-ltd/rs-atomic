// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Atomic Reference
//!
//! Provides an easy-to-use atomic reference type with sensible default memory
//! orderings. Uses `Arc<T>` for thread-safe reference counting.

use arc_swap::{
    ArcSwap,
    Guard,
};
use std::fmt;
use std::sync::Arc;

/// Atomic reference type.
///
/// Provides easy-to-use atomic operations on references with automatic memory
/// ordering selection. Uses `Arc<T>` for thread-safe reference counting.
///
/// # Implementation Details
///
/// This type is backed by `arc_swap::ArcSwap<T>`, which provides lock-free,
/// memory-safe atomic replacement and loading of `Arc<T>` values without
/// exposing raw-pointer lifetime hazards.
///
/// # Features
///
/// - Automatic memory ordering selection
/// - Thread-safe reference counting via `Arc`
/// - Functional update operations
/// - Inline convenience API over `ArcSwap` operations
///
/// `AtomicRef<T>` deliberately does not implement [`Clone`]. Use
/// [`AtomicRef::fork`] to create an independent container explicitly, or use
/// [`crate::ArcAtomicRef`] when owners must share one atomic container.
///
/// ```compile_fail
/// use qubit_atomic::AtomicRef;
///
/// let reference = AtomicRef::from_value(1usize);
/// let _copy = reference.clone();
/// ```
///
/// # Examples
///
/// ```rust
/// use qubit_atomic::AtomicRef;
/// use std::sync::Arc;
///
/// #[derive(Debug, Clone)]
/// struct Config {
///     timeout: u64,
///     max_retries: u32,
/// }
///
/// let config = Arc::new(Config {
///     timeout: 1000,
///     max_retries: 3,
/// });
///
/// let atomic_config = AtomicRef::new(config);
///
/// // Update configuration
/// let new_config = Arc::new(Config {
///     timeout: 2000,
///     max_retries: 5,
/// });
///
/// let old_config = atomic_config.swap(new_config);
/// assert_eq!(old_config.timeout, 1000);
/// assert_eq!(atomic_config.load().timeout, 2000);
/// ```
pub struct AtomicRef<T> {
    /// Lock-free atomic storage for the current shared reference.
    inner: ArcSwap<T>,
}

impl<T> AtomicRef<T> {
    /// Creates a new atomic reference.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial reference.
    ///
    /// # Returns
    ///
    /// An atomic reference initialized to `value`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let data = Arc::new(42);
    /// let atomic = AtomicRef::new(data);
    /// assert_eq!(*atomic.load(), 42);
    /// ```
    #[inline]
    pub fn new(value: Arc<T>) -> Self {
        Self {
            inner: ArcSwap::from(value),
        }
    }

    /// Creates a new atomic reference from an owned value.
    ///
    /// This is a convenience constructor for callers that do not already have
    /// an [`Arc<T>`]. It wraps `value` in [`Arc::new`] and then delegates to
    /// [`new`](Self::new).
    ///
    /// # Parameters
    ///
    /// * `value` - The owned value to store as the initial reference.
    ///
    /// # Returns
    ///
    /// An atomic reference initialized to `Arc::new(value)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    ///
    /// let atomic = AtomicRef::from_value(42);
    /// assert_eq!(*atomic.load(), 42);
    /// ```
    #[inline(always)]
    pub fn from_value(value: T) -> Self {
        Self::new(Arc::new(value))
    }

    /// Gets the current reference.
    ///
    /// # Returns
    ///
    /// A cloned `Arc` pointing to the current value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(42));
    /// let value = atomic.load();
    /// assert_eq!(*value, 42);
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn load(&self) -> Arc<T> {
        self.inner.load_full()
    }

    /// Gets the current reference as an `ArcSwap` guard.
    ///
    /// This is useful for short-lived reads because it avoids cloning the
    /// underlying [`Arc`] on the fast path. Use [`load`](Self::load) when the
    /// caller needs an owned [`Arc<T>`] that can be stored or moved freely.
    ///
    /// # Returns
    ///
    /// A guard pointing to the current `Arc`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(42));
    /// let guard = atomic.load_guard();
    /// assert_eq!(**guard, 42);
    /// ```
    #[inline(always)]
    pub fn load_guard(&self) -> Guard<Arc<T>> {
        self.inner.load()
    }

    /// Sets a new reference.
    ///
    /// # Parameters
    ///
    /// * `value` - The new reference to set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(42));
    /// atomic.store(Arc::new(100));
    /// assert_eq!(*atomic.load(), 100);
    /// ```
    #[inline(always)]
    pub fn store(&self, value: Arc<T>) {
        self.inner.store(value);
    }

    /// Swaps the current reference with a new reference, returning the old
    /// reference.
    ///
    /// # Parameters
    ///
    /// * `value` - The new reference to swap in.
    ///
    /// # Returns
    ///
    /// The old reference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let old = atomic.swap(Arc::new(20));
    /// assert_eq!(*old, 10);
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn swap(&self, value: Arc<T>) -> Arc<T> {
        self.inner.swap(value)
    }

    /// Compares and sets the reference atomically.
    ///
    /// If the current reference equals `current` (by pointer equality), sets
    /// it to `new` and returns `Ok(())`. Otherwise, returns `Err(actual)`
    /// where `actual` is the current reference.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the pointer comparison succeeds and `new` is stored.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed current reference when the
    /// pointer comparison fails. On failure, `new` is not installed.
    ///
    /// # Note
    ///
    /// Comparison uses pointer equality (`Arc::ptr_eq`), not value equality.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let current = atomic.load();
    ///
    /// assert!(atomic.compare_set(&current, Arc::new(20)).is_ok());
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[inline(always)]
    pub fn compare_set(
        &self,
        current: &Arc<T>,
        new: Arc<T>,
    ) -> Result<(), Arc<T>> {
        let prev = Guard::into_inner(self.inner.compare_and_swap(current, new));
        if Arc::ptr_eq(&prev, current) {
            Ok(())
        } else {
            Err(prev)
        }
    }

    /// Compares and exchanges the reference atomically, returning the
    /// previous reference.
    ///
    /// If the current reference equals `current` (by pointer equality), sets
    /// it to `new` and returns the old reference. Otherwise, returns the
    /// actual current reference.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// The reference observed before the operation completed. If it is pointer
    /// equal to `current`, the exchange succeeded and `new` was stored.
    /// Otherwise, it is the actual current reference that prevented the
    /// exchange.
    ///
    /// # Note
    ///
    /// Comparison uses pointer equality (`Arc::ptr_eq`), not value equality.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let current = atomic.load();
    ///
    /// let prev = atomic.compare_and_exchange(&current, Arc::new(20));
    /// assert!(Arc::ptr_eq(&prev, &current));
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[must_use]
    #[inline]
    pub fn compare_and_exchange(
        &self,
        current: &Arc<T>,
        new: Arc<T>,
    ) -> Arc<T> {
        Guard::into_inner(self.inner.compare_and_swap(current, new))
    }

    /// Updates the reference using a function, returning the old reference.
    ///
    /// Internally uses a CAS loop until the update succeeds.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current reference and returns the new
    ///   reference.
    ///
    /// # Returns
    ///
    /// The old reference before the update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let old = atomic.fetch_update(|x| Arc::new(**x * 2));
    /// assert_eq!(*old, 10);
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    pub fn fetch_update<F>(&self, mut f: F) -> Arc<T>
    where
        F: FnMut(&Arc<T>) -> Arc<T>,
    {
        let mut current = self.load();
        loop {
            let new = f(&current);
            match self.compare_set(&current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Updates the reference using a function, returning the new reference.
    ///
    /// Internally uses a CAS loop until the update succeeds.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current reference and returns the new
    ///   reference.
    ///
    /// # Returns
    ///
    /// The reference committed by the successful update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let new = atomic.update_and_get(|x| Arc::new(**x * 2));
    /// assert_eq!(*new, 20);
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    pub fn update_and_get<F>(&self, mut f: F) -> Arc<T>
    where
        F: FnMut(&Arc<T>) -> Arc<T>,
    {
        let mut current = self.load();
        loop {
            let new = f(&current);
            let returned = Arc::clone(&new);
            match self.compare_set(&current, new) {
                Ok(_) => return returned,
                Err(actual) => current = actual,
            }
        }
    }

    /// Conditionally updates the reference using a function.
    ///
    /// Internally uses a pointer-based CAS loop until the update succeeds or
    /// the closure rejects the current reference by returning `None`.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current reference and returns the new
    ///   reference, or `None` to leave the current reference unchanged.
    ///
    /// # Returns
    ///
    /// `Some(old_reference)` when the update succeeds, or `None` when `f`
    /// rejects the observed current reference.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(3));
    /// let old = atomic.try_update(|current| {
    ///     (**current % 2 == 1).then_some(Arc::new(**current + 1))
    /// });
    /// assert_eq!(*old.unwrap(), 3);
    /// assert_eq!(*atomic.load(), 4);
    /// assert!(atomic
    ///     .try_update(|current| {
    ///         (**current % 2 == 1).then_some(Arc::new(**current + 1))
    ///     })
    ///     .is_none());
    /// assert_eq!(*atomic.load(), 4);
    /// ```
    pub fn try_update<F>(&self, mut f: F) -> Option<Arc<T>>
    where
        F: FnMut(&Arc<T>) -> Option<Arc<T>>,
    {
        let mut current = self.load();
        loop {
            let new = f(&current)?;
            match self.compare_set(&current, new) {
                Ok(_) => return Some(current),
                Err(actual) => current = actual,
            }
        }
    }

    /// Conditionally updates the reference using a function, returning the new
    /// reference.
    ///
    /// Internally uses a pointer-based CAS loop until the update succeeds or
    /// the closure rejects the current reference by returning `None`.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current reference and returns the new
    ///   reference, or `None` to leave the current reference unchanged.
    ///
    /// # Returns
    ///
    /// `Some(new_reference)` when the update succeeds, or `None` when `f`
    /// rejects the observed current reference.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(3));
    /// let new = atomic.try_update_and_get(|current| {
    ///     (**current % 2 == 1).then_some(Arc::new(**current + 1))
    /// });
    /// assert_eq!(*new.unwrap(), 4);
    /// assert_eq!(*atomic.load(), 4);
    /// assert!(atomic
    ///     .try_update_and_get(|current| {
    ///         (**current % 2 == 1).then_some(Arc::new(**current + 1))
    ///     })
    ///     .is_none());
    /// assert_eq!(*atomic.load(), 4);
    /// ```
    pub fn try_update_and_get<F>(&self, mut f: F) -> Option<Arc<T>>
    where
        F: FnMut(&Arc<T>) -> Option<Arc<T>>,
    {
        let mut current = self.load();
        loop {
            let new = f(&current)?;
            let returned = Arc::clone(&new);
            match self.compare_set(&current, new) {
                Ok(_) => return Some(returned),
                Err(actual) => current = actual,
            }
        }
    }

    /// Gets a reference to the underlying `ArcSwap`.
    ///
    /// This allows advanced users to access lower-level `ArcSwap` APIs for
    /// custom synchronization strategies.
    ///
    /// # Returns
    ///
    /// A reference to the underlying `arc_swap::ArcSwap<T>`.
    #[must_use]
    #[inline(always)]
    pub fn inner(&self) -> &ArcSwap<T> {
        &self.inner
    }

    /// Creates an independent atomic container from the currently observed
    /// reference.
    ///
    /// The returned container initially stores an [`Arc`] pointing to the same
    /// `T`; this method does not clone `T`. Subsequent `store`, `swap`, and CAS
    /// operations on either container are independent and are not observed by
    /// the other container.
    ///
    /// If this method races with a writer, the new container captures the value
    /// observed by this method's acquire load. It does not guarantee the
    /// globally latest value.
    ///
    /// Use [`crate::ArcAtomicRef`] or `Arc<AtomicRef<T>>` when multiple owners
    /// must operate on the same atomic container.
    ///
    /// # Returns
    ///
    /// A new independent atomic container initialized with the currently
    /// observed shared reference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let original = AtomicRef::from_value(1usize);
    /// let forked = original.fork();
    /// original.store(Arc::new(2));
    ///
    /// assert_eq!(*original.load(), 2);
    /// assert_eq!(*forked.load(), 1);
    /// ```
    #[must_use = "use fork() when you need a separate AtomicRef container"]
    #[inline(always)]
    pub fn fork(&self) -> Self {
        Self::new(self.load())
    }
}

impl<T: fmt::Debug> fmt::Debug for AtomicRef<T> {
    /// Formats the currently loaded reference for debugging.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter receiving the debug representation.
    ///
    /// # Returns
    ///
    /// A formatting result from the formatter.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AtomicRef")
            .field("value", &self.load())
            .finish()
    }
}

impl<T: fmt::Display> fmt::Display for AtomicRef<T> {
    /// Formats the currently loaded reference with display formatting.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter receiving the displayed value.
    ///
    /// # Returns
    ///
    /// A formatting result from the formatter.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
