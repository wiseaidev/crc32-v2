<div align="center">

# CRC32

[![CRC32-V2 Logo](assets/logo.png)](https://github.com/wiseaidotdev/crc32-v2)

[![Crates.io](https://img.shields.io/crates/v/crc32-v2.svg)](https://crates.io/crates/crc32-v2)
[![Docs.rs](https://docs.rs/crc32-v2/badge.svg)](https://docs.rs/crc32-v2)
[![npm](https://img.shields.io/npm/v/crc32-rs.svg)](https://www.npmjs.com/package/crc32-rs)
[![PyPI](https://img.shields.io/pypi/v/crc32-rs.svg)](https://pypi.org/project/crc32-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> `crc32-v2` is a multi-language toolkit providing the fastest port of the CRC-32 algorithm from zlib to Rust, with zero-dependency Python and Node.js native bindings 🗿.

Resurrecting the [`crc32`](https://crates.io/crates/crc32) crate from the ashes.

|                  🦀 Rust                  |                                   🐍 Python                                    |                                 🟩 Node.js                                 |
| :---------------------------------------: | :----------------------------------------------------------------------------: | :------------------------------------------------------------------------: |
|           `cargo add crc32-v2`            |                             `pip install crc32-rs`                             |                           `npm install crc32-rs`                           |
| [Documentation](https://docs.rs/crc32-v2) | [Read PYTHON.md](https://github.com/wiseaidotdev/crc32-v2/blob/main/PYTHON.md) | [Read NODE.md](https://github.com/wiseaidotdev/crc32-v2/blob/main/NODE.md) |

</div>

### Features

- **Standard byte-at-a-time `crc32`**: compatible with zlib, PKZIP, Ethernet, FDDI
- **Four-bytes-at-a-time `crc32_little`**: slicing-by-4, ~2-4x higher throughput on large inputs
- **Big-endian `crc32_big`**: interoperable with big-endian hardware CRC devices
- **Streaming `Digest`**: incremental checksum without buffering the entire payload
- **`crc32_combine`**: merge two independently computed CRCs in O(log n) time
- **Python bindings**: via PyO3 / maturin (`pip install crc32-rs`)
- **Node.js bindings**: via napi-rs (`npm install crc32-rs`)

## Rust Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
crc32-v2 = "0.1.0"
```

Or run:

```sh
cargo add crc32-v2
```

### One-shot checksum

```rust
use crc32_v2::crc32;

fn main() {
    let data = b"Hello, world!";
    println!("CRC-32: {:#010X}", crc32(0, data));  // CRC-32: 0xEBE6C6E6
}
```

### Four-bytes-at-a-time (higher throughput)

```rust
use crc32_v2::byfour::crc32_little;

fn main() {
    let data = b"Hello, world!";
    println!("CRC-32 (little): {:#010X}", crc32_little(0, data));  // CRC-32 (little): 0xEBE6C6E6
}
```

### Streaming checksum via `Digest`

```rust
use crc32_v2::Digest;

fn main() {
    let mut digest = Digest::new();
    digest.update(b"Hello, ");
    digest.update(b"world!");
    println!("CRC-32: {:#010X}", digest.finalize());  // CRC-32: 0xEBE6C6E6
}
```

### Combining two checksums

```rust
use crc32_v2::{crc32, crc32_combine};

fn main() {
    let crc1 = crc32(0, b"Hello, ");
    let crc2 = crc32(0, b"world!");
    let combined = crc32_combine(crc1, crc2, b"world!".len() as u64);
    assert_eq!(combined, crc32(0, b"Hello, world!"));
    println!("Combined: {:#010X}", combined); // Combined: 0xEBE6C6E6
}
```

## Python Usage

See [PYTHON.md](PYTHON.md) for full documentation.

```sh
pip install crc32-rs
```

```python
from crc32_rs import crc32, crc32_little, Digest

print(hex(crc32(b"Hello, world!")))           # 0xebe6c6e6
print(hex(crc32_little(b"Hello, world!")))    # 0xebe6c6e6

d = Digest()
d.update(b"Hello, ")
d.update(b"world!")
print(hex(d.finalize()))                       # 0xebe6c6e6
```

## Node.js Usage

See [NODE.md](NODE.md) for full documentation.

```sh
npm install crc32-rs
```

```javascript
// If installed via npm: const { crc32, crc32Little, Digest } = require('crc32-rs');
// For local development:
const { crc32, crc32Little, Digest } = require(".");

console.log(crc32(Buffer.from("Hello, world!")).toString(16)); // ebe6c6e6
console.log(crc32Little(Buffer.from("Hello, world!")).toString(16)); // ebe6c6e6

const d = new Digest();
d.update(Buffer.from("Hello, "));
d.update(Buffer.from("world!"));
console.log(d.finalize().toString(16)); // ebe6c6e6
```

## Benchmark

Running `cargo bench` measures throughput across five payload sizes. Results on a typical x86-64 machine:

<details>
<summary><code>cargo bench</code></summary>

| **Method**               | **1 B**             | **64 B**             | **1 KiB**           | **64 KiB**           | **1 MiB**           |
| ------------------------ | ------------------- | -------------------- | ------------------- | -------------------- | ------------------- |
| `crc32_v2::crc32`        | ~2.3 ns (421 MiB/s) | ~154 ns (395 MiB/s)  | ~2.8 µs (350 MiB/s) | ~178 µs (352 MiB/s)  | ~2.8 ms (352 MiB/s) |
| `crc32_v2::crc32_little` | ~7.3 ns (131 MiB/s) | ~90.6 ns (673 MiB/s) | ~1.2 µs (814 MiB/s) | ~74.7 µs (837 MiB/s) | ~1.3 ms (771 MiB/s) |
| `crc32_v2::crc32_big`    | ~2.6 ns (371 MiB/s) | ~175 ns (350 MiB/s)  | ~3.2 µs (305 MiB/s) | ~193 µs (324 MiB/s)  | ~3.1 ms (324 MiB/s) |
| `crc32_v2::Digest`       | ~2.2 ns (439 MiB/s) | ~155 ns (393 MiB/s)  | ~2.7 µs (357 MiB/s) | ~173 µs (361 MiB/s)  | ~2.7 ms (364 MiB/s) |
| `crc32fast::hash`        | ~10 ns (96 MiB/s)   | ~20 ns (3.0 GiB/s)   | ~105 ns (9.3 GiB/s) | ~5.4 µs (11.6 GiB/s) | ~86 µs (11.6 GiB/s) |
| `crc32fast::Hasher`      | ~15 ns (64 MiB/s)   | ~36 ns (1.7 GiB/s)   | ~108 ns (9.0 GiB/s) | ~5.5 µs (11.5 GiB/s) | ~87 µs (11.5 GiB/s) |

</details>

> **Key takeaways**
>
> - `crc32_little` achieves ~800 MiB/s throughput for large inputs, making it over 2x faster than the simple byte-at-a-time `crc32` (~350 MiB/s), thanks to the slicing-by-4 algorithm. For tiny inputs (< 16 B), `crc32` is marginally faster due to lower alignment overhead.
> - `crc32_big` falls back to a byte-at-a-time loop and achieves similar throughput to `crc32` (~320 MiB/s).
> - `crc32fast` achieves ~11.6 GiB/s on x86-64 because it uses runtime-detected SIMD hardware acceleration (`pclmulqdq`). For maximum raw throughput on known hardware, prefer `crc32fast`. For pure portability, full control, or embedding in a `no_std` context without CPU feature detection overhead, use `crc32-v2`.

## See Also

- [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt) - the canonical reference for this algorithm.
- [zlib - crc32.c](https://github.com/madler/zlib/blob/master/crc32.c) - the C implementation this crate is ported from.
- [`crc32fast`](https://docs.rs/crc32fast) - SIMD-accelerated CRC-32 for Rust.
- [`crc`](https://docs.rs/crc) - generic CRC computation for many widths and polynomials.
- [IEEE 802.3 CRC-32](https://en.wikipedia.org/wiki/Cyclic_redundancy_check) - Wikipedia overview.
