// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CRC-32 Nano-Benchmarks
//!
//! Measures latency (ns/op) at small buffer sizes (1-512 bytes) where the
//! algorithmic overhead, alignment, loop prologue/epilogue, dominates over
//! bulk throughput. These complement the throughput-oriented `benchmark.rs`.
//!
//! Run with:
//!
//! ```sh
//! cargo bench --bench nano_benchmark
//! ```
//!
//! HTML reports are written to `target/criterion/`.

use crc32_v2::byfour::{crc32_little, crc32_little_8, crc32_little_16};
use crc32_v2::crc32;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

const NANO_SIZES: &[usize] = &[1, 4, 8, 16, 32, 64, 128, 256, 512];

fn generate_data(size: usize) -> Vec<u8> {
    (0u8..=255).cycle().take(size).collect()
}

fn nano_bench_crc32(c: &mut Criterion) {
    let mut group = c.benchmark_group("nano/crc32");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.bench_with_input(BenchmarkId::new("crc32", size), &data, |b, d| {
            b.iter(|| crc32(black_box(0), black_box(d)))
        });
    }

    group.finish();
}

fn nano_bench_crc32_little(c: &mut Criterion) {
    let mut group = c.benchmark_group("nano/crc32_little");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.bench_with_input(BenchmarkId::new("crc32_little", size), &data, |b, d| {
            b.iter(|| crc32_little(black_box(0), black_box(d)))
        });
    }

    group.finish();
}

fn nano_bench_crc32_little_8(c: &mut Criterion) {
    let mut group = c.benchmark_group("nano/crc32_little_8");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.bench_with_input(BenchmarkId::new("crc32_little_8", size), &data, |b, d| {
            b.iter(|| crc32_little_8(black_box(0), black_box(d)))
        });
    }

    group.finish();
}

fn nano_bench_crc32_little_16(c: &mut Criterion) {
    let mut group = c.benchmark_group("nano/crc32_little_16");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.bench_with_input(BenchmarkId::new("crc32_little_16", size), &data, |b, d| {
            b.iter(|| crc32_little_16(black_box(0), black_box(d)))
        });
    }

    group.finish();
}

fn nano_bench_crc32fast(c: &mut Criterion) {
    let mut group = c.benchmark_group("nano/crc32fast");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.bench_with_input(BenchmarkId::new("crc32fast::hash", size), &data, |b, d| {
            b.iter(|| crc32fast::hash(black_box(d)))
        });
    }

    group.finish();
}

fn nano_bench_all_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("nano/all_variants");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);

        group.bench_with_input(BenchmarkId::new("crc32", size), &data, |b, d| {
            b.iter(|| crc32(black_box(0), black_box(d)))
        });
        group.bench_with_input(BenchmarkId::new("crc32_little", size), &data, |b, d| {
            b.iter(|| crc32_little(black_box(0), black_box(d)))
        });
        group.bench_with_input(BenchmarkId::new("crc32_little_8", size), &data, |b, d| {
            b.iter(|| crc32_little_8(black_box(0), black_box(d)))
        });
        group.bench_with_input(BenchmarkId::new("crc32_little_16", size), &data, |b, d| {
            b.iter(|| crc32_little_16(black_box(0), black_box(d)))
        });
        group.bench_with_input(BenchmarkId::new("crc32fast", size), &data, |b, d| {
            b.iter(|| crc32fast::hash(black_box(d)))
        });
    }

    group.finish();
}

criterion_group!(
    nano_benches,
    nano_bench_crc32,
    nano_bench_crc32_little,
    nano_bench_crc32_little_8,
    nano_bench_crc32_little_16,
    nano_bench_crc32fast,
    nano_bench_all_comparison,
);
criterion_main!(nano_benches);
