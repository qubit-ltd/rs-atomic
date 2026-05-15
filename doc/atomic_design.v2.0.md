# Qubit Atomic Design Notes

This document records the design decisions behind `qubit-atomic`. It is not an
API reference. Keep concrete method lists, examples, and signatures in rustdoc
and `README.md`; this document should stay focused on semantics and boundaries.

## Goals

`qubit-atomic` provides a small, ergonomic layer over Rust atomic primitives:

- hide explicit memory ordering for common cases;
- expose high-level operations familiar to users of JDK atomics;
- keep advanced escape hatches available through ordered helpers and `inner()`;
- make return-value semantics explicit and predictable;
- avoid silent wrap where the value models state rather than a pure metric.

The crate intentionally favors a compact public API over exposing every
possible ordering variant. When an operation needs fully custom ordering,
callers can use `inner()`.

## Public Type Model

The public API has four layers:

| Type | Purpose |
| --- | --- |
| `Atomic<T>` | Generic entry point for supported primitive values. |
| `atomic::primitive::*` | Concrete wrappers with `const fn new` for static initialization. |
| `AtomicRef<T>` | Atomic `Arc<T>` reference replacement and pointer-identity CAS. |
| `AtomicCount` / `AtomicSignedCount` | Checked counters for stateful counts and deltas. |
| `ArcAtomic*` | Shared-owner wrappers around the corresponding atomic containers. |

`Atomic<T>` is the preferred default for primitive values. Concrete primitive
wrappers are intentionally kept under `atomic::primitive` so static
initialization remains available without expanding the root API.

## Return-Value Semantics

The method names follow a small set of rules:

| Pattern | Meaning |
| --- | --- |
| `fetch_*` | Return the value observed before the successful operation. |
| `*_and_get` | Return the value committed by the successful operation. |
| `try_*` | Return `None` when the user callback rejects the observed value. |
| `compare_set*` | Return `Result<(), actual>` to make success explicit. |
| `compare_and_exchange` | Return the observed value, matching the convenient JDK-style shape. |
| `compare_and_exchange_weak` | Return `Result<observed, actual>` so weak CAS success is explicit. |

Weak CAS operations can fail spuriously. For that reason,
`compare_and_exchange_weak` must not return a bare value: a value equal to
`current` is not enough to prove success when spurious failure is possible.
The `Result` shape preserves the observed value while making success and
failure unambiguous.

Functional update closures use `FnMut`. CAS loops may call the closure more
than once under contention, so callers must not rely on exactly-once callback
execution.

## Memory Ordering Strategy

The default memory ordering policy is deliberately simple:

| Operation class | Default ordering | Rationale |
| --- | --- | --- |
| `load` | `Acquire` | Observe data published by release operations. |
| `store` | `Release` | Publish prior writes to acquire readers. |
| `swap` and CAS | success `AcqRel`, failure `Acquire` | Standard read-modify-write synchronization. |
| Integer `fetch_add/sub/inc/dec` | `Relaxed` | Commonly used for pure metrics and counters. |
| Integer ordered helpers | caller-provided | Use when the counter value is also a synchronization signal. |
| CAS-loop arithmetic and updates | success `AcqRel`, failure `Acquire` | Preserve synchronization across retry loops. |
| Bitwise and max/min operations | `AcqRel` | Often model flags, thresholds, or shared state. |
| `AtomicCount` operations | success `AcqRel`, failure `Acquire` | Counts are treated as concurrent state signals. |

This policy is a usability default, not a claim that every workload needs these
orderings. Performance-sensitive users can use ordered integer helpers or
`inner()` when they need a different strategy.

## Integer Semantics

Primitive integer operations intentionally follow Rust atomic integer behavior:

- `fetch_add`, `fetch_sub`, `fetch_inc`, and `fetch_dec` wrap on overflow or
  underflow;
- `fetch_mul`, `fetch_div`, `fetch_accumulate`, and `accumulate_and_get` use
  wrapping arithmetic when the provided operation wraps;
- division by zero panics, matching Rust integer division behavior.

Use `AtomicCount` or `AtomicSignedCount` when wrapping would be a bug. Those
types model state-oriented counts and return new values for transition logic.

## Floating-Point Semantics

Floating-point atomics are implemented with raw-bit integer atomics:

- `Atomic<f32>` stores `f32::to_bits()` in `AtomicU32`;
- `Atomic<f64>` stores `f64::to_bits()` in `AtomicU64`;
- CAS compares raw bit patterns, not `PartialEq`.

This means `0.0` and `-0.0` do not CAS-match, and NaN payloads must match
exactly. Use `compare_set` or `compare_set_weak` when the caller needs an
explicit success indicator.

Floating-point arithmetic uses CAS loops. It is convenient, not a replacement
for numerically stable accumulation in high-contention workloads.

## Reference Semantics

`AtomicRef<T>` is backed by `arc_swap::ArcSwap<T>` and stores `Arc<T>` values.
Its CAS operations compare pointer identity, not `T: Eq` value equality.

`AtomicRef<T>::clone()` creates a new atomic container initialized with the
current reference. It does not create another handle to the same container. Use
`ArcAtomicRef<T>` when clones should share one atomic container.

`load_guard()` is available for short-lived reads that can avoid cloning the
`Arc` on the fast path. Use `load()` when the caller needs an owned `Arc<T>`.

## API Surface Policy

Add public methods only when they satisfy at least one of these criteria:

- they clarify an important return-value semantic;
- they prevent a common misuse;
- they match an established operation family already present in the crate;
- they expose necessary functionality that cannot be expressed through
  existing methods without losing correctness.

Do not add aliases such as `add_fetch` or `update_fetch` when the existing name
already expresses the return value clearly. Prefer the `*_and_get` family for
new-value returns because it is readable and consistent with the JDK-style API.

## Documentation Policy

The documentation roles are:

- rustdoc: authoritative API signatures and runnable examples;
- README: user-facing overview and common workflows;
- this design note: durable design rationale and semantic boundaries.

Avoid duplicating full API tables here. If a code change requires editing many
places in this document, the document is probably becoming too detailed again.

## Test Policy

The test suite should cover:

- return-value semantics for each operation family;
- checked vs wrapping behavior;
- raw-bit floating-point CAS behavior;
- pointer-identity reference CAS behavior;
- callback retry behavior for CAS loops;
- Markdown Rust examples that are not covered by rustdoc.

Markdown example tests should compile README snippets against the local crate
through a path dependency. This catches documentation drift without turning the
design document into another API source of truth.
