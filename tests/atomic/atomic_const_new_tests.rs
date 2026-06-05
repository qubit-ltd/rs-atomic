// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_atomic::atomic::primitive::{
    AtomicBool,
    AtomicF32,
    AtomicF64,
    AtomicI8,
    AtomicI16,
    AtomicI32,
    AtomicI64,
    AtomicI128,
    AtomicIsize,
    AtomicU8,
    AtomicU16,
    AtomicU32,
    AtomicU64,
    AtomicU128,
    AtomicUsize,
};

static BOOL_ATOMIC: AtomicBool = AtomicBool::new(true);
static I8_ATOMIC: AtomicI8 = AtomicI8::new(42);
static U8_ATOMIC: AtomicU8 = AtomicU8::new(42);
static I16_ATOMIC: AtomicI16 = AtomicI16::new(42);
static U16_ATOMIC: AtomicU16 = AtomicU16::new(42);
static I32_ATOMIC: AtomicI32 = AtomicI32::new(42);
static U32_ATOMIC: AtomicU32 = AtomicU32::new(42);
static I64_ATOMIC: AtomicI64 = AtomicI64::new(42);
static U64_ATOMIC: AtomicU64 = AtomicU64::new(42);
static I128_ATOMIC: AtomicI128 = AtomicI128::new(42);
static U128_ATOMIC: AtomicU128 = AtomicU128::new(42);
static ISIZE_ATOMIC: AtomicIsize = AtomicIsize::new(42);
static USIZE_ATOMIC: AtomicUsize = AtomicUsize::new(42);
static F32_ATOMIC: AtomicF32 = AtomicF32::new(3.5);
static F64_ATOMIC: AtomicF64 = AtomicF64::new(3.5);

#[test]
fn test_atomic_bool_new_is_const() {
    assert!(BOOL_ATOMIC.load());
}

#[test]
fn test_atomic_integer_new_is_const() {
    assert_eq!(I8_ATOMIC.load(), 42);
    assert_eq!(U8_ATOMIC.load(), 42);
    assert_eq!(I16_ATOMIC.load(), 42);
    assert_eq!(U16_ATOMIC.load(), 42);
    assert_eq!(I32_ATOMIC.load(), 42);
    assert_eq!(U32_ATOMIC.load(), 42);
    assert_eq!(I64_ATOMIC.load(), 42);
    assert_eq!(U64_ATOMIC.load(), 42);
    assert_eq!(I128_ATOMIC.load(), 42);
    assert_eq!(U128_ATOMIC.load(), 42);
    assert_eq!(ISIZE_ATOMIC.load(), 42);
    assert_eq!(USIZE_ATOMIC.load(), 42);
}

#[test]
fn test_atomic_float_new_is_const() {
    assert_eq!(F32_ATOMIC.load().to_bits(), 3.5_f32.to_bits());
    assert_eq!(F64_ATOMIC.load().to_bits(), 3.5_f64.to_bits());
}
