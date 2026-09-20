<div align="center">

# ⚡ CRC32-V2

[![CRC32-V2 Logo](https://raw.githubusercontent.com/wiseaidev/crc32-v2/refs/heads/main/assets/logo.png)](https://github.com/wiseaidev/crc32-v2)

[![Crates.io](https://img.shields.io/crates/v/crc32-v2.svg)](https://crates.io/crates/crc32-v2)
[![Docs.rs](https://docs.rs/crc32-v2/badge.svg)](https://docs.rs/crc32-v2)
[![Build Status](https://github.com/wiseaidev/crc32-v2/actions/workflows/rust.yml/badge.svg)](https://github.com/wiseaidev/crc32-v2/actions/workflows/rust.yml)
[![PyPI](https://img.shields.io/pypi/v/crc32-rs.svg)](https://pypi.org/project/crc32-rs)
[![npm](https://img.shields.io/npm/v/crc32-rs.svg)](https://www.npmjs.com/package/crc32-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> The fastest pure-software, `no_std`-compatible CRC-32 implementation in Rust, slicing-by-16 tables, zero heap allocation in the hot path, and native Python / Node.js bindings, all written in **100% safe Rust** 🗿.

[![Banger Banner](https://raw.githubusercontent.com/wiseaidev/crc32-v2/refs/heads/main/assets/banner.png)](https://github.com/wiseaidev/crc32-v2)

</div>

## 🚀 Installation

| Platform            | Command / Link                                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------- |
| **Rust**            | `cargo add crc32-v2`                                                                                      |
| **Rust (no_std)**   | `cargo add crc32-v2 --no-default-features`                                                                |
| **Python**          | `pip install crc32-rs`                                                                                    |
| **Node.js**         | `npm install crc32-rs`                                                                                    |
| **Debian / Ubuntu** | [`gh release download`](https://github.com/wiseaidev/crc32-v2/releases), see [PACKAGING.md](PACKAGING.md) |
| **RHEL / Fedora**   | [`gh release download`](https://github.com/wiseaidev/crc32-v2/releases), see [PACKAGING.md](PACKAGING.md) |

## 🔭 Features

| Feature                | Description                                                                           |
| ---------------------- | ------------------------------------------------------------------------------------- |
| **`no_std` + `alloc`** | Embeddable in bare-metal, WASM, and embedded targets, no standard library required    |
| **100% safe Rust**     | `#![forbid(unsafe_code)]` enforced crate-wide; zero `unsafe` blocks                   |
| **`crc32`**            | Byte-at-a-time baseline (~343 MiB/s); compatible with zlib, PKZIP, Ethernet, and FDDI |
| **`crc32_little`**     | Slicing-by-4 (~833 MiB/s); **2.4×** faster than byte-at-a-time                        |
| **`crc32_little_8`**   | Slicing-by-8 (~1,004 MiB/s); **2.9×** faster than byte-at-a-time                      |
| **`crc32_little_16`**  | Slicing-by-16 (**~1,282 MiB/s**); **3.7×** fastest pure-software path                 |
| **`crc32_big`**        | Big-endian (unreflected) variant for hardware CRC controllers                         |
| **`crc32_combine`**    | Merge two independently computed CRCs in O(log n) via GF(2) matrix squaring           |
| **Streaming `Digest`** | Incremental checksum with zero-copy buffering                                         |
| **Build-time tables**  | `crc32-codegen` subcrate generates all 16 lookup tables at compile time               |
| **Python bindings**    | Via PyO3 / maturin (`pip install crc32-rs`)                                           |
| **Node.js bindings**   | Via napi-rs (`npm install crc32-rs`)                                                  |

## 🦀 Rust

### One-shot checksum

```rust
use crc32_v2::crc32;

let data = b"Hello, world!";
println!("CRC-32: {:#010X}", crc32(0, data));  // CRC-32: 0xEBE6C6E6
assert_eq!(crc32(0, b"123456789"), 0xCBF43926);
```

### Slicing-by-16 (fastest pure-software path, ~1,282 MiB/s)

```rust
use crc32_v2::byfour::crc32_little_16;

let data: Vec<u8> = (0..1_048_576).map(|i| i as u8).collect();
let crc = crc32_little_16(0, &data);
println!("CRC-32: {crc:#010X}");
assert_eq!(crc32_little_16(0, b"123456789"), 0xCBF43926);
```

### Slicing-by-8 (~1,004 MiB/s)

```rust
use crc32_v2::byfour::crc32_little_8;

let crc = crc32_little_8(0, b"Hello, world!");
assert_eq!(crc, 0xEBE6_C6E6);
```

### Slicing-by-4 (~833 MiB/s)

```rust
use crc32_v2::byfour::crc32_little;

let crc = crc32_little(0, b"Hello, world!");
assert_eq!(crc, 0xEBE6_C6E6);
```

### Combining two checksums without the original data

```rust
use crc32_v2::{crc32, crc32_combine};

let crc1 = crc32(0, b"Hello, ");
let crc2 = crc32(0, b"world!");
let combined = crc32_combine(crc1, crc2, b"world!".len() as u64);
assert_eq!(combined, crc32(0, b"Hello, world!"));
```

### Streaming `Digest`

```rust
use crc32_v2::Digest;

let mut digest = Digest::new();
digest.update(b"Hello, ");
digest.update(b"world!");
println!("CRC-32: {:#010X}", digest.finalize()); // 0xEBE6C6E6
let bytes: [u8; 4] = digest.digest();            // [0xEB, 0xE6, 0xC6, 0xE6]
```

### `no_std` usage

```toml
[dependencies]
crc32-v2 = { version = "0.1.2", default-features = false }
```

```rust,ignore
#![no_std]

use crc32_v2::{crc32, byfour::crc32_little_16, Digest};

let crc = crc32_little_16(0, b"embedded payload");
assert!(crc != 0);
```

## 🐍 Python

See [PYTHON.md](PYTHON.md) for the full API reference.

```sh
pip install crc32-rs
```

```python
from crc32_rs import crc32, crc32_little_16, crc32_bytes, crc32_hex, Digest

print(hex(crc32(b"Hello, world!")))             # 0xebe6c6e6
print(hex(crc32_little_16(b"Hello, world!")))   # 0xebe6c6e6
print(crc32_bytes(b"Hello, world!").hex())      # ebe6c6e6
print(crc32_hex(b"Hello, world!"))              # ebe6c6e6

d = Digest()
d.update(b"Hello, ")
d.update(b"world!")
print(hex(d.finalize()))   # 0xebe6c6e6
print(d.digest().hex())    # ebe6c6e6
print(repr(d))             # Digest(crc=0xEBE6C6E6)
```

## 🟩 Node.js

See [NODE.md](NODE.md) for the full API reference.

```sh
npm install crc32-rs
```

```js
const { crc32, crc32Little16, Digest } = require("crc32-rs");

console.log(crc32(Buffer.from("Hello, world!")).toString(16)); // ebe6c6e6
console.log(crc32Little16(Buffer.from("Hello, world!")).toString(16)); // ebe6c6e6
```

## 📊 Benchmarks

Measured with `cargo bench` (`lto="fat"`, `opt-level=3`, `codegen-units=1`, `overflow-checks=false`) on Linux x86-64 (Rust 1.94.1). Full methodology in [BENCHMARKS.md](BENCHMARKS.md).

### Throughput across payload sizes

| Method                      |      1 B |      64 B |      1 KiB |        64 KiB |          1 MiB |       Throughput |
| --------------------------- | -------: | --------: | ---------: | ------------: | -------------: | ---------------: |
| `crc32_v2::crc32`           |     2 ns |    167 ns |   2,856 ns |    188,134 ns |   2,916,562 ns |       ~343 MiB/s |
| `crc32_v2::crc32_little`    |     3 ns |     73 ns |   1,086 ns |     82,129 ns |   1,199,953 ns |       ~833 MiB/s |
| `crc32_v2::crc32_little_8`  |     3 ns |     61 ns |     922 ns |     56,486 ns |   1,004,746 ns |     ~1,004 MiB/s |
| `crc32_v2::crc32_little_16` | **3 ns** | **39 ns** | **753 ns** | **48,101 ns** | **781,535 ns** | **~1,282 MiB/s** |
| `crc32fast::hash` (SIMD)    |    11 ns |     20 ns |     102 ns |      5,802 ns |      89,204 ns |    ~11,300 MiB/s |

### Cross-library comparison: 1 MiB payload

| Library           | Method            |           Time |       Throughput | Implementation            |
| ----------------- | ----------------- | -------------: | ---------------: | ------------------------- |
| `crc32fast`       | `hash`            |      89,204 ns |    ~11,300 MiB/s | SIMD (`pclmulqdq`) / Rust |
| `zlib-rs`         | `crc32`           |      96,968 ns |    ~10,390 MiB/s | SIMD / Rust               |
| Python (`zlib`)   | `zlib.crc32`      |     355,270 ns |     ~2,815 MiB/s | SIMD / C extension        |
| **`crc32-v2`**    | `crc32_little_16` | **781,535 ns** | **~1,282 MiB/s** | **Pure safe Rust**        |
| Python (`crcmod`) | `crcmod`          |   2,865,836 ns |       ~349 MiB/s | Non-SIMD / C extension    |

### Python overhead: small inputs

| Payload | Python `zlib.crc32` | Rust `crc32-v2` |   Speedup |
| ------- | ------------------: | --------------: | --------: |
| 1 B     |             ~297 ns |           ~2 ns | **~148×** |
| 64 B    |             ~301 ns |          ~39 ns | **~7.7×** |

> **Key takeaways**
>
> - `crc32_little_16` achieves **~1,282 MiB/s**, the fastest pure-software, safe-Rust CRC-32 path, **3.7× faster** than byte-at-a-time.
> - For inputs ≤ 1 byte, `crc32` (2 ns) wins because slicing variants pay an alignment-prologue cost.
> - `crc32fast` achieves ~11 GiB/s via runtime hardware SIMD. For `no_std`, embedded, or WASM targets, use `crc32-v2`.
> - Python interpreter overhead dominates for small payloads: Rust is **148× faster** for 1-byte inputs.

## 🗂 Module Structure

| Module        | Contents                                                                     |
| ------------- | ---------------------------------------------------------------------------- |
| [`tables`]    | `crc32`: byte-at-a-time CRC-32 function                                      |
| [`byfour`]    | `crc32_little` / `crc32_little_8` / `crc32_little_16` / `crc32_big`          |
| [`combine`]   | `crc32_combine`: O(log n) GF(2) matrix merging                               |
| [`digest`]    | `Digest`: streaming incremental interface                                    |
| `crc32tables` | 16 LE slicing tables + BE table (generated by `crc32-codegen` at build time) |

## 🔒 Safety

`#![forbid(unsafe_code)]` is enforced at the crate root. Every byte of the implementation, 16-table generation, slicing loops, GF(2) matrix arithmetic, and the streaming `Digest`, is written in safe Rust. The `no_std` build path carries the same safety guarantees.

## 📚 Further Reading

- [BENCHMARKS.md](BENCHMARKS.md): Full benchmark methodology and measured results
- [RUST.md](RUST.md): Complete Rust API reference with complexity table
- [PYTHON.md](PYTHON.md): Python bindings guide
- [NODE.md](NODE.md): Node.js bindings guide
- [PACKAGING.md](PACKAGING.md): Debian and RPM packaging guide
- [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
- [zlib: crc32.c](https://github.com/madler/zlib/blob/master/crc32.c): the C source this crate is ported from
- [`crc32fast`](https://docs.rs/crc32fast): SIMD-accelerated CRC-32 for Rust
- [`crc`](https://docs.rs/crc): generic CRC for many widths and polynomials

> [!NOTE]
> This project is a successor of the awesome [`crc32`](https://crates.io/crates/crc32) crate created by the kawaii engineers at Microsoft 👉👈.

## 📄 License

Licensed under the [MIT License](LICENSE).
