/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! Macro for implementing the hidden atomic value marker trait.

/// Implements the hidden atomic value marker for a supported primitive type.
macro_rules! impl_atomic_value {
    ($value_type:ty, $primitive_type:ty, $inner_type:ty) => {
        impl sealed::Sealed for $value_type {}

        impl AtomicValue for $value_type {
            type Primitive = $primitive_type;
            type Inner = $inner_type;

            #[inline]
            fn new_primitive(value: Self) -> Self::Primitive {
                <$primitive_type>::new(value)
            }

            #[inline]
            fn inner(primitive: &Self::Primitive) -> &Self::Inner {
                primitive.inner()
            }
        }
    };
}
