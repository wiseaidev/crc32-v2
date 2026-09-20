# CRC32-V2 Benchmarks

Comprehensive benchmark results, methodology, and comparisons for `crc32-v2`.

## Environment

```
OS:       Linux x86-64
Profile:  opt-level=3, lto="fat", codegen-units=1, overflow-checks=false, panic=abort
Toolchain: Rust stable (1.94.1)
Command:  cargo bench --bench benchmark -- --output-format bencher
```

## Running the Benchmarks

```sh
# Full throughput benchmark suite (Criterion, HTML report in target/criterion/)
cargo bench --bench benchmark

# Nano-benchmarks: latency-focused, 1-512 byte payloads
cargo bench --bench nano_benchmark
cargo bench --bench compare_benchmark

# Quick single run
cargo bench --bench benchmark -- --quick
```

## Throughput

Results from `cargo bench --bench benchmark -- --output-format bencher`:

| Method                      | 1 B   | 64 B   | 1 KiB    | 64 KiB     | 1 MiB        |
| --------------------------- | ----- | ------ | -------- | ---------- | ------------ |
| `crc32_v2::crc32`           | 2 ns  | 167 ns | 2,856 ns | 188,134 ns | 2,916,562 ns |
| `crc32_v2::crc32_little`    | 3 ns  | 73 ns  | 1,086 ns | 82,129 ns  | 1,199,953 ns |
| `crc32_v2::crc32_little_8`  | 3 ns  | 61 ns  | 922 ns   | 56,486 ns  | 1,004,746 ns |
| `crc32_v2::crc32_little_16` | 3 ns  | 39 ns  | 753 ns   | 48,101 ns  | 781,535 ns   |
| `crc32fast::hash` (SIMD)    | 11 ns | 20 ns  | 102 ns   | 5,802 ns   | 89,204 ns    |

### Throughput (MiB/s) for large inputs (1 MiB payload)

| Method                      | Throughput       | Speedup vs `crc32` |
| --------------------------- | ---------------- | ------------------ |
| `crc32_v2::crc32`           | ~343 MiB/s       | 1× (baseline)      |
| `crc32_v2::crc32_little`    | ~833 MiB/s       | **2.4×**           |
| `crc32_v2::crc32_little_8`  | ~1,004 MiB/s     | **2.9×**           |
| `crc32_v2::crc32_little_16` | **~1,282 MiB/s** | **3.7×**           |
| `crc32fast::hash` (SIMD)    | ~11,300 MiB/s    | 33×                |

> `crc32_little_16` is the **fastest pure-software, 100%-safe-Rust CRC-32 implementation** in this crate,
> delivering ~3.7× the throughput of the byte-at-a-time baseline, without any SIMD or unsafe code.

## Nano-Benchmarks

Results from `cargo bench --bench benchmark -- --output-format bencher` (ns/iter):

| Size   | `crc32`      | `crc32_little` | `crc32_little_8` | `crc32_little_16` | `crc32fast` |
| ------ | ------------ | -------------- | ---------------- | ----------------- | ----------- |
| 1 B    | 2 ns         | 3 ns           | 3 ns             | 3 ns              | 11 ns       |
| 64 B   | 167 ns       | 73 ns          | 61 ns            | 39 ns             | 20 ns       |
| 1 KiB  | 2,856 ns     | 1,086 ns       | 922 ns           | 753 ns            | 102 ns      |
| 64 KiB | 188,134 ns   | 82,129 ns      | 56,486 ns        | 48,101 ns         | 5,802 ns    |
| 1 MiB  | 2,916,562 ns | 1,199,953 ns   | 1,004,746 ns     | 781,535 ns        | 89,204 ns   |

> **For inputs ≤ 1 byte**, the byte-at-a-time `crc32` is fastest (2 ns) because slicing variants
> pay a small alignment-prologue overhead. `crc32fast` has the highest small-input overhead (11 ns)
> due to its SIMD dispatch branching.

## Cross-Library Comparison (Measured)

Comparing `crc32-v2` (`crc32_little_16`) against other popular libraries on 1 MiB payload:

| Library     | Function          | Time (1 MiB)  | Throughput    | Type                           |
| ----------- | ----------------- | ------------- | ------------- | ------------------------------ |
| `crc32fast` | `hash`            | ~89,204 ns    | ~11,300 MiB/s | SIMD / Rust                    |
| `zlib-rs`   | `crc32`           | ~96,968 ns    | ~10,390 MiB/s | SIMD / Rust                    |
| Python (C)  | `zlib.crc32`      | ~355,270 ns   | ~2,815 MiB/s  | SIMD / C Extension             |
| `crc32-v2`  | `crc32_little_16` | ~751,328 ns   | ~1,282 MiB/s  | Pure Safe Rust (Slicing-by-16) |
| Python (C)  | `crcmod`          | ~2,865,836 ns | ~349 MiB/s    | Non-SIMD / C Extension         |

> **Notes:**
>
> - `crc32fast` and `zlib-rs` achieve 10+ GiB/s because they use feature-detected hardware SIMD (`pclmulqdq` instruction on x86-64).
> - Python's `zlib.crc32` delegates to the system C `zlib`, which also typically employs SIMD acceleration for CRC-32 on modern hardware.
> - `crc32-v2` provides ~1.3 GiB/s **with zero SIMD and zero unsafe code.** This guarantees maximum portability and avoids CPU-feature detection overhead.

## Python Latency vs Pure Rust (Small Inputs)

While Python's `zlib.crc32` is fast for very large payloads due to underlying SIMD C code, it suffers from heavy Python interpreter overhead for small inputs. A direct comparison for 1 B and 64 B payloads:

| Payload | Python `zlib.crc32` (ns) | Rust `crc32-v2` (ns) | Rust speedup |
| ------- | ------------------------ | -------------------- | ------------ |
| 1 B     | ~297 ns                  | ~2 ns                | **~148×**    |
| 64 B    | ~301 ns                  | ~39 ns               | **~7.7×**    |

When calculating CRCs of many small payloads, moving the loop entirely into Rust yields tremendous speedups. For massive 1-MiB payloads, the boundary crossing cost is negligible, so `zlib.crc32` processes it efficiently (but wait till you see Rust `crc32fast` vs Python!).

## Optimization Techniques Applied

| Technique                                       | Effect                                                   |
| ----------------------------------------------- | -------------------------------------------------------- |
| **Slicing-by-16** (16 tables, 16 bytes/step)    | ~3.7× faster than byte-at-a-time for large inputs        |
| **Slicing-by-8** (8 tables, 8 bytes/step)       | ~2.9× faster than byte-at-a-time for large inputs        |
| **Zero heap allocation**                        | Eliminated `Vec<u32>` intermediate in old `crc32_little` |
| **Unrolled outer loops** (8× per 128 B)         | Reduces loop overhead; better ILP                        |
| **`#[inline(always)]`** on `fold4/fold8/fold16` | Forces inlining of hot inner-step functions              |
| **Fat LTO** (`lto = "fat"`)                     | Cross-crate inlining; eliminates function-call overhead  |
| **`overflow-checks = false`**                   | Removes branch overhead on integer arithmetic            |
| **`codegen-units = 1`**                         | Single LLVM codegen unit for maximum optimization        |
| **`panic = "abort"`**                           | Removes unwinding landing pads; tighter code             |
| **Separate `[profile.bench]`**                  | All above settings apply to `cargo bench` as well        |

## Why Not SIMD?

`crc32fast` achieves ~11 GiB/s via runtime-detected `pclmulqdq` on x86-64. This crate
intentionally does **not** use SIMD or `unsafe` code in order to:

1. Enforce `#![forbid(unsafe_code)]` globally.
1. Remain fully portable (WASM, ARM, RISC-V, embedded, etc.).
1. Work in `no_std` environments without CPU-feature detection.

For maximum throughput on known x86-64 hardware, use `crc32fast`. For portability,
`no_std` contexts, or full control, use `crc32-v2`.
