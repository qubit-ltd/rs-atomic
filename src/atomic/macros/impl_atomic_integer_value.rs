// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Macro for implementing hidden integer marker operations.

/// Implements hidden integer marker operations for a supported primitive type.
macro_rules! impl_atomic_integer_value {
    ($value_type:ty, $primitive_type:ty, $inner_type:ty) => {
        impl_atomic_value!($value_type, $primitive_type, $inner_type);

        impl AtomicIntegerValue for $value_type {
            #[inline(always)]
            fn fetch_inc(primitive: &Self::Primitive) -> Self {
                primitive.fetch_inc()
            }

            #[inline(always)]
            fn fetch_inc_with_ordering(
                primitive: &Self::Primitive,
                ordering: Ordering,
            ) -> Self {
                primitive.fetch_inc_with_ordering(ordering)
            }

            #[inline(always)]
            fn fetch_dec(primitive: &Self::Primitive) -> Self {
                primitive.fetch_dec()
            }

            #[inline(always)]
            fn fetch_dec_with_ordering(
                primitive: &Self::Primitive,
                ordering: Ordering,
            ) -> Self {
                primitive.fetch_dec_with_ordering(ordering)
            }

            #[inline(always)]
            fn fetch_add_with_ordering(
                primitive: &Self::Primitive,
                value: Self,
                ordering: Ordering,
            ) -> Self {
                primitive.fetch_add_with_ordering(value, ordering)
            }

            #[inline(always)]
            fn fetch_sub_with_ordering(
                primitive: &Self::Primitive,
                value: Self,
                ordering: Ordering,
            ) -> Self {
                primitive.fetch_sub_with_ordering(value, ordering)
            }

            #[inline(always)]
            fn fetch_and(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_and(value)
            }

            #[inline(always)]
            fn fetch_or(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_or(value)
            }

            #[inline(always)]
            fn fetch_xor(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_xor(value)
            }

            #[inline(always)]
            fn fetch_not(primitive: &Self::Primitive) -> Self {
                primitive.fetch_not()
            }

            #[inline(always)]
            fn fetch_accumulate<F>(
                primitive: &Self::Primitive,
                value: Self,
                f: F,
            ) -> Self
            where
                F: FnMut(Self, Self) -> Self,
            {
                primitive.fetch_accumulate(value, f)
            }

            #[inline(always)]
            fn accumulate_and_get<F>(
                primitive: &Self::Primitive,
                value: Self,
                f: F,
            ) -> Self
            where
                F: FnMut(Self, Self) -> Self,
            {
                primitive.accumulate_and_get(value, f)
            }

            #[inline(always)]
            fn fetch_max(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_max(value)
            }

            #[inline(always)]
            fn fetch_min(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_min(value)
            }
        }
    };
}
