# Qubit Atomic

[![Rust CI](https://github.com/qubit-ltd/rs-atomic/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-atomic/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-atomic/coverage-badge.json)](https://qubit-ltd.github.io/rs-atomic/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-atomic.svg?color=blue)](https://crates.io/crates/qubit-atomic)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

User-friendly atomic operations wrapper providing JDK-like atomic API for Rust.

## Overview

Qubit Atomic is a comprehensive atomic operations library that provides easy-to-use atomic types with reasonable default memory orderings, similar to Java's `java.util.concurrent.atomic` package. It hides the complexity of memory ordering, keeps primitive forwarding wrappers thin, and allows advanced users to access underlying types for fine-grained control.

## Design Goals

- **Ease of Use**: Hides memory ordering complexity with reasonable defaults
- **Completeness**: Provides high-level operations similar to JDK atomic classes
- **Safety**: Guarantees memory safety and thread safety
- **Performance**: Thin primitive forwarding wrappers, with explicit
  benchmarks for higher-level operations
- **Flexibility**: Exposes narrowly scoped ordered integer RMW helpers and
  `inner()` for advanced users
- **Simplicity**: Default APIs keep common cases free of explicit ordering
  parameters

## Features

### 🔢 **Generic Atomic Primitive Types**
- **Integer Specializations**: `Atomic<i8>`, `Atomic<u8>`, `Atomic<i16>`, `Atomic<u16>`, `Atomic<i32>`, `Atomic<u32>`, `Atomic<i64>`, `Atomic<u64>`, `Atomic<i128>`, `Atomic<u128>`, `Atomic<isize>`, `Atomic<usize>`
- **128-bit Portability**: `Atomic<i128>` and `Atomic<u128>` use
  `portable-atomic`; native lock-free support is target-dependent, and its
  fallback may use locks on targets without suitable atomic instructions
- **Boolean Specialization**: `Atomic<bool>` with set, clear, negate, logical AND/OR/XOR, and conditional CAS helpers
- **Floating-Point Specializations**: `Atomic<f32>` and `Atomic<f64>` with arithmetic operations implemented through CAS loops
- **Rich Operations**: increment, decrement, add, subtract, multiply, divide, bitwise operations, max/min
- **Functional Updates**: `fetch_update`, `update_and_get`, `try_update`, `try_update_and_get`, `fetch_accumulate`, `accumulate_and_get`
- **Const Initialization Escape Hatch**: concrete wrappers such as `atomic::primitive::AtomicBool`, `AtomicU8`, and `AtomicF32` expose `const fn new` for static initialization when the generic `Atomic<T>` constructor cannot be used in const contexts

### 🔢 **`AtomicCount` and `AtomicSignedCount`**
- **`AtomicCount`**: non-negative count for active tasks, in-flight requests, and resource usage
- **`AtomicSignedCount`**: signed count for deltas, balances, backlog, and offsets
- **No-Wrap Semantics**: checked updates that panic or return `None` instead of wrapping
- **Zero-Transition Logic**: `inc`, `dec`, `add`, and `sub` return the new value

### 🔗 **Atomic Reference Type**
- **AtomicRef<T>**: Thread-safe atomic reference using `Arc<T>`
- **Reference Updates**: Atomic swap and CAS operations
- **Guarded Loads**: `load_guard()` for short-lived reads without cloning on the fast path
- **Functional Updates**: Transform references atomically with `fetch_update` / `update_and_get` or conditionally with `try_update` / `try_update_and_get`

### 🤝 **Shared-Owner Convenience Wrappers**
- **`ArcAtomic<T>`**: convenience newtype around `Arc<Atomic<T>>`
- **`ArcAtomicRef<T>`**: convenience newtype around `Arc<AtomicRef<T>>`
- **`ArcAtomicCount` / `ArcAtomicSignedCount`**: shared-owner wrappers for the count types
- **Shared Container Clone**: cloning an `ArcAtomic*` value shares the same atomic container

### 🎯 **Focused Public API**
- **Atomic<T>**: Generic entry point for primitive atomic values
- **`atomic::primitive::*`**: concrete primitive wrappers for `const fn new` use cases
- **AtomicRef<T>**: Atomic `Arc<T>` reference wrapper
- **Explicit Reference Fork**: `AtomicRef::fork()` snapshots the currently
  observed `Arc<T>` into an independent atomic container
- **`AtomicCount` / `AtomicSignedCount`**: checked state-oriented semantics (no silent wrap)
- **`ArcAtomic*` wrappers**: ergonomic shared ownership without spelling `Arc<...>` at every use site

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-atomic = "0.14"
```

## Quick Start

### Specifying the value type `T`

`Atomic<T>` is generic over the primitive value type. Rust usually infers `T` from the argument to [`Atomic::new`](https://docs.rs/qubit-atomic/latest/qubit_atomic/struct.Atomic.html#method.new), but literals such as `0` can be ambiguous across integer widths.

In those cases, pick `T` explicitly using a [turbofish](https://doc.rust-lang.org/book/appendix-02-operators.html#the-turbofish) on the constructor, or by annotating the variable:

```rust
use qubit_atomic::Atomic;

let wide: Atomic<u64> = Atomic::new(0);
assert_eq!(wide.load(), 0u64);

let narrow = Atomic::<i16>::new(0);
assert_eq!(narrow.load(), 0i16);
```

### Const initialization

Use `Atomic<T>` for normal code. The generic constructor is the intended entry point because it keeps the public API compact and lets one type cover all primitive specializations.

Rust stable does not currently allow `Atomic<T>::new` to call the hidden trait constructor in a `const fn`. When you need a `static` or another const-initialized atomic value, use the concrete wrappers under `atomic::primitive`:

```rust
use qubit_atomic::atomic::primitive::{
    AtomicBool,
    AtomicU32,
};

static READY: AtomicBool = AtomicBool::new(false);
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
```

### Example: concurrent `Atomic<i32>`

```rust
use qubit_atomic::Atomic;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(Atomic::<i32>::new(0));
    let mut handles = vec![];

    // Spawn 10 threads, each increments counter 1000 times
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_inc();
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify result
    assert_eq!(counter.load(), 10000);
    println!("Final count: {}", counter.load());
}
```

### `AtomicCount` and `AtomicSignedCount`

Use `Atomic<T>` for pure metrics. Use `AtomicCount` when the
count is part of concurrent state, such as active work or termination checks.

```rust
use qubit_atomic::{
    AtomicCount,
    AtomicSignedCount,
};

fn main() {
    let active_tasks = AtomicCount::zero();

    active_tasks.inc();
    assert!(!active_tasks.is_zero());

    if active_tasks.dec() == 0 {
        println!("all active tasks are finished");
    }

    let backlog_delta = AtomicSignedCount::zero();
    assert_eq!(backlog_delta.add(5), 5);
    assert_eq!(backlog_delta.sub(8), -3);
    assert!(backlog_delta.is_negative());
}
```

### Shared-owner wrappers

Use the `ArcAtomic*` wrappers when the atomic container itself is shared across
threads or components. Their `clone()` operation clones the outer `Arc`, so all
clones observe and update the same container.

```rust
use qubit_atomic::{
    ArcAtomic,
    ArcAtomicCount,
    ArcAtomicRef,
    ArcAtomicSignedCount,
};
use std::sync::Arc;
use std::thread;

fn main() {
    let requests = ArcAtomic::new(0usize);
    let worker_requests = requests.clone();

    let handle = thread::spawn(move || {
        worker_requests.fetch_inc();
    });
    handle.join().expect("worker should finish");

    assert_eq!(requests.load(), 1);
    assert_eq!(requests.strong_count(), 1);

    let active_tasks = ArcAtomicCount::zero();
    let shared_tasks = active_tasks.clone();
    assert_eq!(shared_tasks.inc(), 1);
    assert_eq!(active_tasks.get(), 1);

    let backlog = ArcAtomicSignedCount::zero();
    let shared_backlog = backlog.clone();
    assert_eq!(shared_backlog.sub(3), -3);
    assert_eq!(backlog.get(), -3);

    let config = ArcAtomicRef::from_value(String::from("v1"));
    let same_config = config.clone();
    same_config.store(Arc::new(String::from("v2")));
    assert_eq!(config.load().as_str(), "v2");
}
```

### CAS Loop

```rust
use qubit_atomic::Atomic;

fn increment_even_only(atomic: &Atomic<i32>) -> Result<i32, &'static str> {
    let mut current = atomic.load();
    loop {
        // Only increment even values
        if current % 2 != 0 {
            return Err("Value is odd");
        }

        let new = current + 2;
        match atomic.compare_set(current, new) {
            Ok(_) => return Ok(new),
            Err(actual) => current = actual, // Retry
        }
    }
}

fn main() {
    let atomic = Atomic::<i32>::new(10);
    match increment_even_only(&atomic) {
        Ok(new_value) => println!("Successfully incremented to: {}", new_value),
        Err(e) => println!("Failed: {}", e),
    }
    assert_eq!(atomic.load(), 12);
}
```

### Functional Updates

```rust
use qubit_atomic::Atomic;

fn main() {
    let atomic = Atomic::<i32>::new(10);

    // Update using a function (returns old value)
    let old_value = atomic.fetch_update(|x| {
        if x < 100 {
            x * 2
        } else {
            x
        }
    });

    assert_eq!(old_value, 10);
    assert_eq!(atomic.load(), 20);
    println!("Updated value: {}", atomic.load());

    // Update and return the committed new value
    let new_value = atomic.update_and_get(|x| x + 5);
    assert_eq!(new_value, 25);
    assert_eq!(atomic.load(), 25);

    // Conditional update and return the committed new value
    let accepted_new = atomic.try_update_and_get(|x| (x < 100).then_some(x + 5));
    assert_eq!(accepted_new, Some(30));
    assert_eq!(atomic.load(), 30);

    // Accumulate operation (returns old value)
    let old_result = atomic.fetch_accumulate(5, |a, b| a + b);
    assert_eq!(old_result, 30);
    assert_eq!(atomic.load(), 35);

    // Accumulate and return the committed new value
    let accumulated = atomic.accumulate_and_get(5, |a, b| a + b);
    assert_eq!(accumulated, 40);
    assert_eq!(atomic.load(), 40);
    println!("Accumulated value: {}", atomic.load());
}
```

### Atomic Reference

```rust
use qubit_atomic::AtomicRef;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Config {
    timeout: u64,
    max_retries: u32,
}

fn main() {
    let config = Arc::new(Config {
        timeout: 1000,
        max_retries: 3,
    });

    let atomic_config = AtomicRef::new(config);

    // Update configuration
    let new_config = Arc::new(Config {
        timeout: 2000,
        max_retries: 5,
    });

    let old_config = atomic_config.swap(new_config);
    println!("Old config: {:?}", old_config);
    println!("New config: {:?}", atomic_config.load());

    // Update using a function (returns old value)
    let old = atomic_config.fetch_update(|current| {
        Arc::new(Config {
            timeout: current.timeout * 2,
            max_retries: current.max_retries + 1,
        })
    });

    println!("Previous config: {:?}", old);
    println!("Updated config: {:?}", atomic_config.load());

    // Update and return the committed new reference
    let updated = atomic_config.update_and_get(|current| {
        Arc::new(Config {
            timeout: current.timeout + 500,
            max_retries: current.max_retries + 1,
        })
    });
    println!("Committed config: {:?}", updated);

    // Short-lived read without cloning the Arc on the fast path
    let snapshot = atomic_config.load_guard();
    println!("Snapshot config: {:?}", snapshot);

    // Conditional update; returns the committed new reference or None if rejected
    let accepted = atomic_config.try_update_and_get(|current| {
        (current.timeout < 10_000).then_some(Arc::new(Config {
            timeout: current.timeout + 1000,
            max_retries: current.max_retries,
        }))
    });
    assert!(accepted.is_some());
}
```

### Boolean Flag

```rust
use qubit_atomic::Atomic;
use std::sync::Arc;

struct Service {
    running: Arc<Atomic<bool>>,
}

impl Service {
    fn new() -> Self {
        Self {
            running: Arc::new(Atomic::<bool>::new(false)),
        }
    }

    fn start(&self) {
        // Only start if not already running
        if self.running.set_if_false(true).is_ok() {
            println!("Service started successfully");
        } else {
            println!("Service is already running");
        }
    }

    fn stop(&self) {
        // Only stop if currently running
        if self.running.set_if_true(false).is_ok() {
            println!("Service stopped successfully");
        } else {
            println!("Service is already stopped");
        }
    }

    fn is_running(&self) -> bool {
        self.running.load()
    }
}

fn main() {
    let service = Service::new();

    service.start();
    assert!(service.is_running());

    service.start(); // Duplicate start will fail

    service.stop();
    assert!(!service.is_running());

    service.stop(); // Duplicate stop will fail
}
```

### Floating-Point Atomics

```rust
use qubit_atomic::Atomic;
use std::sync::Arc;
use std::thread;

fn main() {
    let sum = Arc::new(Atomic::<f32>::new(0.0));
    let mut handles = vec![];

    // Spawn 10 threads, each adds 100 times
    for _ in 0..10 {
        let sum = sum.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                sum.fetch_add(0.01);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Note: Due to floating-point precision, result may not be exactly 10.0
    let result = sum.load();
    println!("Sum: {:.6}", result);
    println!("Error: {:.6}", (result - 10.0).abs());
}
```

## API Reference

### Common Operations

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `new(value)` | Create new atomic | - |
| `load()` | Load current value | Acquire |
| `store(value)` | Store new value | Release |
| `swap(value)` | Swap value, return old | AcqRel |
| `compare_set(current, new)` | CAS operation, return explicit success/failure | AcqRel/Acquire |
| `compare_and_exchange(current, new)` | CAS operation, return observed value | AcqRel/Acquire |
| `fetch_update(f)` | Functional update, return old | AcqRel/Acquire |
| `update_and_get(f)` | Functional update, return new | AcqRel/Acquire |
| `try_update(f)` | Conditional functional update, return `Option<old>` | AcqRel/Acquire |
| `try_update_and_get(f)` | Conditional functional update, return `Option<new>` | AcqRel/Acquire |
| `inner()` | Access underlying backend type | - |

### Primitive Weak CAS Operations

These methods are available on primitive `Atomic<T>` specializations. They are
not available on `AtomicRef<T>`, which exposes strong pointer CAS only.

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `compare_set_weak(current, new)` | Weak CAS, return explicit success/failure | AcqRel/Acquire |
| `compare_and_exchange_weak(current, new)` | Weak CAS, return `Ok(observed)` or `Err(actual)` | AcqRel/Acquire |

### Integer Operations

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `fetch_inc()` | Post-increment, return old | Relaxed |
| `fetch_dec()` | Post-decrement, return old | Relaxed |
| `fetch_add(delta)` | Post-add, return old | Relaxed |
| `fetch_sub(delta)` | Post-subtract, return old | Relaxed |
| `fetch_inc_with_ordering(ordering)` | Post-increment, return old | Caller-provided |
| `fetch_dec_with_ordering(ordering)` | Post-decrement, return old | Caller-provided |
| `fetch_add_with_ordering(delta, ordering)` | Post-add, return old | Caller-provided |
| `fetch_sub_with_ordering(delta, ordering)` | Post-subtract, return old | Caller-provided |
| `fetch_mul(factor)` | Post-multiply, return old | AcqRel (CAS loop) |
| `fetch_div(divisor)` | Post-divide, return old | AcqRel (CAS loop) |
| `fetch_and(value)` | Bitwise AND, return old | AcqRel |
| `fetch_or(value)` | Bitwise OR, return old | AcqRel |
| `fetch_xor(value)` | Bitwise XOR, return old | AcqRel |
| `fetch_not()` | Bitwise NOT, return old | AcqRel |
| `fetch_max(value)` | Atomic max, return old | AcqRel |
| `fetch_min(value)` | Atomic min, return old | AcqRel |
| `fetch_update(f)` | Functional update, return old | AcqRel/Acquire |
| `update_and_get(f)` | Functional update, return new | AcqRel/Acquire |
| `try_update(f)` | Conditional functional update, return `Option<old>` | AcqRel/Acquire |
| `try_update_and_get(f)` | Conditional functional update, return `Option<new>` | AcqRel/Acquire |
| `fetch_accumulate(x, f)` | Accumulate, return old | AcqRel/Acquire |
| `accumulate_and_get(x, f)` | Accumulate, return new | AcqRel/Acquire |

Primitive integer operations intentionally use wrapping arithmetic on overflow
and underflow, matching Rust atomic integer semantics. Use `AtomicCount` or
`AtomicSignedCount` when overflow or underflow should be rejected.
The `_with_ordering` variants are available only for integer read-modify-write
counter arithmetic, for cases where the counter value is also used as a
synchronization signal.

### `AtomicCount` / `AtomicSignedCount` operations

| Method | `AtomicCount` | `AtomicSignedCount` | Memory Ordering | Description |
|--------|-----------------|-----------------------|-----------------|-------------|
| `new(value)` | `usize` | `isize` | - | Create a count |
| `zero()` | Yes | Yes | - | Create a zero value |
| `get()` | `usize` | `isize` | Acquire | Read the current value |
| `is_zero()` | Yes | Yes | Acquire | Check whether the value is zero |
| `is_positive()` | Yes | Yes | Acquire | Check whether the value is positive |
| `is_negative()` | No | Yes | Acquire | Check whether the value is negative |
| `inc()` | Yes | Yes | AcqRel/Acquire | Increment by one, return new value |
| `dec()` | Panic on underflow | Allows negative values | AcqRel/Acquire | Decrement by one, return new value |
| `add(delta)` | Panic on overflow | Panic on overflow/underflow | AcqRel/Acquire | Add delta, return new value |
| `sub(delta)` | Panic on underflow | Panic on overflow/underflow | AcqRel/Acquire | Subtract delta, return new value |
| `try_add(delta)` | `None` on overflow | `None` on overflow/underflow | AcqRel/Acquire | Checked add |
| `try_dec()` | `None` at zero | No | AcqRel/Acquire (`AtomicCount` only) | Checked decrement |
| `try_sub(delta)` | `None` on underflow | `None` on overflow/underflow | AcqRel/Acquire | Checked subtract |

### Shared-owner wrapper operations

The `ArcAtomic*` wrappers dereference to their underlying atomic container, so
you can call operations such as `load`, `fetch_inc`, `store`, `inc`, and `sub`
directly on the wrapper.

| Method | Available On | Description |
|--------|--------------|-------------|
| `new(value)` | `ArcAtomic<T>`, `ArcAtomicCount`, `ArcAtomicSignedCount` | Create a new shared wrapper from an initial value |
| `new(Arc<T>)` | `ArcAtomicRef<T>` | Create a shared atomic reference from an existing `Arc<T>` |
| `from_value(value)` | `ArcAtomicRef<T>` | Create a shared atomic reference from an owned value |
| `from_atomic(...)` | `ArcAtomic<T>` | Wrap an existing `Atomic<T>` |
| `from_atomic_ref(...)` | `ArcAtomicRef<T>` | Wrap an existing `AtomicRef<T>` |
| `from_count(...)` | `ArcAtomicCount`, `ArcAtomicSignedCount` | Wrap an existing count container |
| `from_arc(arc)` | All `ArcAtomic*` wrappers | Wrap an existing `Arc<...>` container |
| `as_arc()` | All `ArcAtomic*` wrappers | Borrow the underlying `Arc<...>` |
| `into_arc()` | All `ArcAtomic*` wrappers | Consume the wrapper and return the underlying `Arc<...>` |
| `strong_count()` | All `ArcAtomic*` wrappers | Return the number of strong `Arc` owners |

### Boolean Operations

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `fetch_set()` | Set to true, return old | AcqRel |
| `fetch_clear()` | Set to false, return old | AcqRel |
| `fetch_not()` | Negate, return old | AcqRel |
| `fetch_and(value)` | Logical AND, return old | AcqRel |
| `fetch_or(value)` | Logical OR, return old | AcqRel |
| `fetch_xor(value)` | Logical XOR, return old | AcqRel |
| `set_if_false(new)` | CAS if false | AcqRel/Acquire |
| `set_if_true(new)` | CAS if true | AcqRel/Acquire |

### Floating-Point Operations

| Method | Description | Memory Ordering |
|--------|-------------|-----------------|
| `fetch_add(delta)` | Atomic add, return old | AcqRel (CAS loop) |
| `fetch_sub(delta)` | Atomic subtract, return old | AcqRel (CAS loop) |
| `fetch_mul(factor)` | Atomic multiply, return old | AcqRel (CAS loop) |
| `fetch_div(divisor)` | Atomic divide, return old | AcqRel (CAS loop) |
| `fetch_update(f)` | Functional update, return old | AcqRel/Acquire |
| `update_and_get(f)` | Functional update, return new | AcqRel/Acquire |
| `try_update(f)` | Conditional functional update, return `Option<old>` | AcqRel/Acquire |
| `try_update_and_get(f)` | Conditional functional update, return `Option<new>` | AcqRel/Acquire |

Floating-point CAS operations (`compare_set`, `compare_and_exchange`, and weak
variants) compare raw `to_bits()` representations, not `PartialEq`. Values such
as `0.0` and `-0.0` compare equal but do not CAS-match, and NaN payload bits
must match exactly. Use `compare_set` when you need an explicit success result,
or compare `to_bits()` values yourself.

## Memory Ordering Strategy

| Operation Type | Default Ordering | Reason |
|---------------|------------------|--------|
| **Pure Read** (`load()`) | `Acquire` | If the load observes a release sequence, subsequent operations observe data published before that release; it does not guarantee the globally newest value |
| **Pure Write** (`store()`) | `Release` | Publish prior writes to an acquire operation that observes this store or its release sequence |
| **Read-Modify-Write** (`swap()`, CAS) | `AcqRel` | Acquire from an observed release and publish prior writes through the successful modification |
| **`Atomic<T>` counter arithmetic** (`fetch_inc()`, `fetch_dec()`, `fetch_add()`, `fetch_sub()`) | `Relaxed` | Pure metrics; no need to sync other data |
| **Ordered integer counter arithmetic** (`fetch_*_with_ordering`) | Caller-provided | State-signal counters that need explicit synchronization |
| **CAS-based arithmetic and updates** (`fetch_mul()`, `fetch_div()`, `fetch_update()`, `update_and_get()`, `try_update()`, `try_update_and_get()`, `fetch_accumulate()`, `accumulate_and_get()`) | `AcqRel` / `Acquire` | CAS loop standard semantics |
| **`AtomicCount` / `AtomicSignedCount`** (`inc()`, `dec()`) | CAS loop | Values used as concurrent state signals |
| **Bitwise Operations** (`fetch_and()`, `fetch_or()`) | `AcqRel` | Usually used for flag synchronization |
| **Max/Min Operations** (`fetch_max()`, `fetch_min()`) | `AcqRel` | Often used with threshold checks |

### Advanced Usage: Explicit Ordering and Direct Access

For integer counter arithmetic that needs synchronization semantics, use the
ordered variants before reaching for `inner()`:

```rust
use std::sync::atomic::Ordering;
use qubit_atomic::Atomic;

let atomic = Atomic::<i32>::new(0);

// Common case: use the wrapper default
let value = atomic.load();

// State-signal counter: keep the wrapper while choosing ordering explicitly
atomic.fetch_add_with_ordering(1, Ordering::AcqRel);

// Lowest-level escape hatch: direct backend access
let value = atomic.inner().load(Ordering::Relaxed);
atomic.inner().store(42, Ordering::Release);
```

## Comparison with JDK

| Feature | JDK | Qubit Atomic | Notes |
|---------|-----|---------------|-------|
| **Basic Types** | 3 types | `Atomic<T>` specializations; `atomic::primitive::*` for const initialization | Rust supports more integer, floating-point, boolean, and counter use cases |
| **Memory Ordering** | Implicit (volatile) | Defaults + ordered integer RMW helpers + `inner()` optional | Rust more flexible |
| **Weak CAS** | `weakCompareAndSet` | `compare_set_weak` on primitive `Atomic<T>` | Equivalent |
| **Reference Type** | `AtomicReference<V>` | `AtomicRef<T>` | Rust uses `Arc<T>` |
| **`AtomicCount` / `AtomicSignedCount`** | Manual composition | `AtomicCount`, `AtomicSignedCount` | Non-negative / signed counts for state tracking |
| **Shared Ownership** | Usually object references | `ArcAtomic<T>`, `ArcAtomicRef<T>`, `ArcAtomicCount`, `ArcAtomicSignedCount` | Convenience wrappers for shared atomic containers |
| **Nullability** | Allows `null` | Not provided by `AtomicRef<T>` | Use `arc_swap::ArcSwapOption<T>` or another synchronization type when atomic nullable references are required |
| **Bitwise Operations** | Partial support | Full support | Rust more powerful |
| **Max/Min Operations** | Java 9+ support | Supported | Equivalent |
| **API Count** | ~20 methods/type | ~29 methods/type | Rust provides more convenience methods |

## Performance Considerations

### Thin Primitive Forwarding

Primitive wrappers use `#[repr(transparent)]` and `#[inline]`. In optimized
builds, simple forwarding methods are intended to compile down to the
corresponding backend atomic operations:

```rust
use qubit_atomic::Atomic;
use std::sync::atomic::Ordering;

// Our wrapper
let atomic = Atomic::<i32>::new(0);
let value = atomic.load();

// Equivalent backend operation
let atomic = std::sync::atomic::AtomicI32::new(0);
let value = atomic.load(Ordering::Acquire);
```

This scope does not cover operations with inherent work: floating-point and
checked-count updates use CAS retry loops, while `AtomicRef<T>` retains
`ArcSwap`'s reference-counting, reclamation, and retry costs. Measure on the
target workload instead of assuming identical code generation.

### When to Use `inner()`

Use the default API when its ordering and operation semantics fit. Reach for
`inner()` only when:
- A required ordering is not exposed by the focused wrapper API
- A measured hot path benefits from a different valid ordering
- A low-level algorithm needs precise backend control
- Interoperating with code that directly uses standard library or `portable-atomic` backend types

The Criterion benchmark target compares representative wrapper operations with
equivalent `std` and `arc-swap` baselines. Results remain target- and
workload-specific.

## Dependencies

Runtime dependencies are intentionally small:

- `arc-swap` powers `AtomicRef<T>`.
- `portable-atomic` provides the stable backend for `Atomic<i128>` and
  `Atomic<u128>`; its lock-free guarantees and fallback strategy depend on the
  compilation target.

## Testing

```bash
# Core API with the default empty feature set
cargo test --no-default-features

# Core API plus regex validation
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-atomic](https://github.com/qubit-ltd/rs-atomic)
