// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Standard trait implementations for [`super::Atomic<T>`].

use std::fmt;

use super::super::atomic_value::AtomicValue;
use super::Atomic;

impl<T> Default for Atomic<T>
where
    T: AtomicValue + Default,
{
    /// Creates an atomic with [`Default::default`] as the initial value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<i32>::default();
    /// assert_eq!(atomic.load(), 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for Atomic<T>
where
    T: AtomicValue,
{
    /// Converts `value` into an [`Atomic`] via [`Atomic::new`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::from(42i32);
    /// assert_eq!(atomic.load(), 42);
    /// ```
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> fmt::Debug for Atomic<T>
where
    T: AtomicValue + fmt::Debug,
{
    /// Formats the loaded value as `Atomic { value: ... }`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(7);
    /// assert!(format!("{:?}", atomic).contains("7"));
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Atomic").field("value", &self.load()).finish()
    }
}

impl<T> fmt::Display for Atomic<T>
where
    T: AtomicValue + fmt::Display,
{
    /// Formats the loaded value using its [`Display`](fmt::Display)
    /// implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(42);
    /// assert_eq!(format!("{}", atomic), "42");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
