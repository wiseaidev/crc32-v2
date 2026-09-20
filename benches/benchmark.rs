// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CRC-32 Benchmarks
//!
//! Measures throughput (MiB/s and GiB/s) and latency (ns/op) across multiple
//! input sizes and all algorithm variants including comparisons against
//! `crc32fast` and the `crc` crate. Run with:
//!
//! ```sh
//! cargo bench --bench benchmark
//! ```
//!
//! HTML reports are written to `target/criterion/`.

use crc32_v2::byfour::{crc32_big, crc32_little, crc32_little_8, crc32_little_16};
use crc32_v2::{Digest, crc32};
use crc32fast::Hasher;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

const SIZES: &[usize] = &[1, 64, 1024, 65536, 1_048_576];

fn generate_data(size: usize) -> Vec<u8> {
    (0u8..=255).cycle().take(size).collect()
}

fn bench_crc32(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32");

    for &size in SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("crc32_v2::crc32", size), &data, |b, d| {
            b.iter(|| crc32(black_box(0), black_box(d)))
        });
    }

    group.finish();
}

fn bench_crc32_little(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32_little");

    for &size in SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("crc32_v2::crc32_little", size),
            &data,
            |b, d| b.iter(|| crc32_little(black_box(0), black_box(d))),
        );
    }

    group.finish();
}

fn bench_crc32_little_8(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32_little_8");

    for &size in SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("crc32_v2::crc32_little_8", size),
            &data,
            |b, d| b.iter(|| crc32_little_8(black_box(0), black_box(d))),
        );
    }

    group.finish();
}

fn bench_crc32_little_16(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32_little_16");

    for &size in SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("crc32_v2::crc32_little_16", size),
            &data,
            |b, d| b.iter(|| crc32_little_16(black_box(0), black_box(d))),
        );
    }

    group.finish();
}

fn bench_crc32_big(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32_big");

    for &size in SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("crc32_v2::crc32_big", size),
            &data,
            |b, d| b.iter(|| crc32_big(black_box(0), black_box(d))),
        );
    }

    group.finish();
}

fn bench_digest(c: &mut Criterion) {
    let mut group = c.benchmark_group("digest");

    for &size in SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("crc32_v2::Digest", size), &data, |b, d| {
            b.iter(|| {
                let mut digest = black_box(Digest::new());
                digest.update(black_box(d));
                digest.finalize()
            })
        });
    }

    group.finish();
}

fn bench_crc32fast(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32fast");

    for &size in SIZES {
        let data = generate_data(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("crc32fast::hash", size), &data, |b, d| {
            b.iter(|| crc32fast::hash(black_box(d)))
        });
        group.bench_with_input(
            BenchmarkId::new("crc32fast::Hasher", size),
            &data,
            |b, d| {
                b.iter(|| {
                    let mut hasher = black_box(Hasher::new());
                    hasher.update(black_box(d));
                    hasher.finalize()
                })
            },
        );
    }

    group.finish();
}

fn bench_all_variants_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_variants");

    for &size in SIZES {
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
        group.bench_with_input(BenchmarkId::new("crc32fast", size), &data, |b, d| {
            b.iter(|| crc32fast::hash(black_box(d)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_crc32,
    bench_crc32_little,
    bench_crc32_little_8,
    bench_crc32_little_16,
    bench_crc32_big,
    bench_digest,
    bench_crc32fast,
    bench_all_variants_comparison,
);
criterion_main!(benches);
