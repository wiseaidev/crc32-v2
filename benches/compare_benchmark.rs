// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CRC-32 Cross-Library Nano-Benchmarks
//!
//! Measures latency (ns/op) across small **and** large buffer sizes, comparing:
//!
//! - `crc32_v2`: all pure-software variants (slicing-by-1/4/8/16)
//! - `zlib-rs`: Rust port of zlib's CRC-32 (uses SIMD when available)
//! - `crc32fast`: SIMD-accelerated (pclmulqdq on x86-64)
//!
//! A companion Python script (`benchmarks/python_bench.py`) measures the
//! equivalent Python implementations (`zlib.crc32`, `crcmod`) and prints a
//! combined comparison table.
//!
//! Run with:
//!
//! ```sh
//! cargo bench --bench compare_benchmark
//! python3 benchmarks/python_bench.py
//! ```
//!
//! HTML reports land in `target/criterion/`.

use crc32_v2::byfour::{crc32_little, crc32_little_8, crc32_little_16};
use crc32_v2::crc32;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;
use zlib_rs::crc32::crc32 as zlib_rs_crc32;

const NANO_SIZES: &[usize] = &[1, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 65536, 1_048_576];

fn generate_data(size: usize) -> Vec<u8> {
    (0u8..=255).cycle().take(size).collect()
}

fn bench_crc32_v2_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare/crc32_v2");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));

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
    }

    group.finish();
}

fn bench_zlib_rs(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare/zlib_rs");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("zlib_rs::crc32", size), &data, |b, d| {
            b.iter(|| zlib_rs_crc32(black_box(0), black_box(d)))
        });
    }

    group.finish();
}

fn bench_crc32fast(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare/crc32fast");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in NANO_SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("crc32fast::hash", size), &data, |b, d| {
            b.iter(|| crc32fast::hash(black_box(d)))
        });
    }

    group.finish();
}

fn bench_all_rust_at_key_sizes(c: &mut Criterion) {
    let key_sizes: &[usize] = &[1, 64, 1024, 65536, 1_048_576];

    let mut group = c.benchmark_group("compare/all_rust");
    group.measurement_time(Duration::from_secs(5));
    group.warm_up_time(Duration::from_secs(2));

    for &size in key_sizes {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("crc32_v2::crc32", size), &data, |b, d| {
            b.iter(|| crc32(black_box(0), black_box(d)))
        });
        group.bench_with_input(
            BenchmarkId::new("crc32_v2::crc32_little", size),
            &data,
            |b, d| b.iter(|| crc32_little(black_box(0), black_box(d))),
        );
        group.bench_with_input(
            BenchmarkId::new("crc32_v2::crc32_little_8", size),
            &data,
            |b, d| b.iter(|| crc32_little_8(black_box(0), black_box(d))),
        );
        group.bench_with_input(
            BenchmarkId::new("crc32_v2::crc32_little_16", size),
            &data,
            |b, d| b.iter(|| crc32_little_16(black_box(0), black_box(d))),
        );
        group.bench_with_input(BenchmarkId::new("zlib_rs::crc32", size), &data, |b, d| {
            b.iter(|| zlib_rs_crc32(black_box(0), black_box(d)))
        });
        group.bench_with_input(BenchmarkId::new("crc32fast::hash", size), &data, |b, d| {
            b.iter(|| crc32fast::hash(black_box(d)))
        });
    }

    group.finish();
}

criterion_group!(
    compare_benches,
    bench_crc32_v2_all,
    bench_zlib_rs,
    bench_crc32fast,
    bench_all_rust_at_key_sizes,
);
criterion_main!(compare_benches);
