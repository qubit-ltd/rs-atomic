/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Atomic Performance Benchmarks
//!
//! Benchmarks for atomic operations to measure performance.

use qubit_atomic::Atomic;
use std::hint::black_box;
use std::sync::Arc;
use std::thread;

/// Command used to run this custom benchmark target.
const RUN_COMMAND: &str = "cargo bench --bench atomic_bench";

/// Environment variable that forces benchmark execution in debug/test profile.
const FORCE_RUN_ENV: &str = "RUN_ATOMIC_BENCH";

/// Benchmark scenario names and descriptions printed for `--list`.
const BENCHMARKS: &[(&str, &str)] = &[
    (
        "single_threaded_increment",
        "Atomic<i32>::fetch_inc in one thread, 1,000,000 operations",
    ),
    (
        "multi_threaded_increment",
        "Atomic<i32>::fetch_inc across 10 threads, 1,000,000 total operations",
    ),
    (
        "compare_and_swap",
        "Atomic<i32>::compare_set loop, 1,000,000 operations",
    ),
    (
        "functional_update",
        "Atomic<i32>::fetch_update loop, 1,000,000 operations",
    ),
    (
        "read_operations",
        "Atomic<i32>::load loop, 10,000,000 operations",
    ),
];

/// Runs the custom benchmark executable.
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.iter().any(|arg| is_help_arg(arg)) {
        print_help();
        return;
    }
    if args.iter().any(|arg| is_list_arg(arg)) {
        print_benchmark_list();
        return;
    }
    if should_skip_for_test_profile() {
        print_test_profile_skip();
        return;
    }

    run_benchmarks();
}

/// Returns whether `arg` requests help output.
fn is_help_arg(arg: &str) -> bool {
    matches!(arg, "-h" | "--help")
}

/// Returns whether `arg` requests benchmark list output.
fn is_list_arg(arg: &str) -> bool {
    matches!(arg, "--list")
}

/// Returns whether benchmark execution should be skipped in the test profile.
fn should_skip_for_test_profile() -> bool {
    cfg!(debug_assertions) && std::env::var_os(FORCE_RUN_ENV).is_none()
}

/// Prints usage information for this custom benchmark target.
fn print_help() {
    println!("Atomic benchmark runner");
    println!();
    println!("Usage:");
    println!("  {RUN_COMMAND}");
    println!("  {RUN_COMMAND} -- --list");
    println!("  {RUN_COMMAND} -- --help");
    println!();
    println!("Set {FORCE_RUN_ENV}=1 to force execution in debug/test profile.");
}

/// Prints the benchmark scenarios supported by this target.
fn print_benchmark_list() {
    println!("atomic_bench scenarios:");
    for (name, description) in BENCHMARKS {
        println!("  {name}: {description}");
    }
    println!();
    println!("Run with: {RUN_COMMAND}");
}

/// Prints why this benchmark was skipped under `cargo test --all-targets`.
fn print_test_profile_skip() {
    println!("atomic_bench skipped in debug/test profile.");
    println!("Run `{RUN_COMMAND}` to execute benchmarks.");
}

/// Runs all benchmark scenarios and prints timing results.
fn run_benchmarks() {
    println!("=== Atomic Performance Benchmarks ===\n");

    // Benchmark 1: Single-threaded increment
    println!("1. Single-threaded Increment (1,000,000 operations):");
    let counter = Atomic::<i32>::new(0);
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        black_box(counter.fetch_inc());
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );
    println!("   Final value: {}", black_box(counter.load()));

    // Benchmark 2: Multi-threaded increment
    println!("\n2. Multi-threaded Increment (10 threads, 100,000 ops each):");
    let counter = Arc::new(Atomic::<i32>::new(0));
    let start = std::time::Instant::now();
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                black_box(counter.fetch_inc());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );
    println!("   Final value: {}", counter.load());

    // Benchmark 3: Compare-and-swap
    println!("\n3. Compare-and-Swap (1,000,000 operations):");
    let counter = Atomic::<i32>::new(0);
    let start = std::time::Instant::now();
    for i in 0..1_000_000 {
        while black_box(counter.compare_set(i, i + 1)).is_err() {
            // Retry on failure
        }
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );
    println!("   Final value: {}", black_box(counter.load()));

    // Benchmark 4: Functional update
    println!("\n4. Functional Update (1,000,000 operations):");
    let counter = Atomic::<i32>::new(0);
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        black_box(counter.fetch_update(|x| black_box(x + 1)));
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );
    println!("   Final value: {}", black_box(counter.load()));

    // Benchmark 5: Read operations
    println!("\n5. Read Operations (10,000,000 operations):");
    let counter = Atomic::<i32>::new(42);
    let start = std::time::Instant::now();
    let mut sum = 0i64;
    for _ in 0..10_000_000 {
        sum = black_box(sum + black_box(&counter).load() as i64);
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        10_000_000.0 / duration.as_secs_f64()
    );
    println!("   Sum: {} (to prevent optimization)", sum);

    println!("\n=== Benchmarks completed ===");
}
