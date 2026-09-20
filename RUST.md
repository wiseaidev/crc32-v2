# crc32-v2 Rust Documentation 🦀

The `crc32-v2` Rust crate provides a pure-software, 100%-safe-Rust CRC-32
implementation with slicing-by-4/8/16 high-throughput variants, a streaming
`Digest` type, O(log n) CRC combining, and native Python / Node.js bindings.

## 📦 Installation

```toml
[dependencies]
crc32-v2 = "0.1.2"
```

## 🗂 Module Structure

| Module        | Contents                                                            |
| ------------- | ------------------------------------------------------------------- |
| [`tables`]    | `crc32`: byte-at-a-time CRC-32 (IEEE 802.3, ~343 MiB/s)             |
| [`byfour`]    | `crc32_little` / `crc32_little_8` / `crc32_little_16` / `crc32_big` |
| [`combine`]   | `crc32_combine`: O(log n) GF(2) matrix merging                      |
| [`digest`]    | `Digest`: streaming incremental checksum                            |
| `crc32tables` | 16 LE slicing tables + BE table (build-time, `crc32-codegen`)       |

## 🛠 Core Functions

### `crc32`: One-shot checksum

```rust
use crc32_v2::crc32;

assert_eq!(crc32(0, b"123456789"), 0xCBF43926);

let crc = crc32(crc32(0, b"Hello, "), b"world!");
assert_eq!(crc, crc32(0, b"Hello, world!"));
```

### `crc32_little_16`: Fastest pure-software path (~1.3 GiB/s)

```rust
use crc32_v2::byfour::crc32_little_16;

assert_eq!(crc32_little_16(0, b"123456789"), 0xCBF43926);

let data: Vec<u8> = (0u8..=255).cycle().take(1_048_576).collect();
let crc = crc32_little_16(0, &data);
println!("CRC-32: {crc:#010X}");
```

### `crc32_little_8`: Slicing-by-8 (~1 GiB/s)

```rust
use crc32_v2::byfour::crc32_little_8;

assert_eq!(crc32_little_8(0, b"123456789"), 0xCBF43926);
```

### `crc32_little`: Slicing-by-4 (~833 MiB/s)

```rust
use crc32_v2::byfour::crc32_little;

assert_eq!(crc32_little(0, b"123456789"), 0xCBF43926);
```

### `crc32_big`: Big-endian (unreflected)

```rust
use crc32_v2::byfour::crc32_big;

let empty: &[u8] = &[];
assert_eq!(crc32_big(0, empty), 0);

let full = crc32_big(0, b"Hello, world!");
let chained = crc32_big(crc32_big(0, b"Hello, "), b"world!");
assert_eq!(full, chained);
```

### `crc32_combine`: Merge two CRCs in O(log n)

```rust
use crc32_v2::{crc32, crc32_combine};

let crc1 = crc32(0, b"Hello, ");
let crc2 = crc32(0, b"world!");
let combined = crc32_combine(crc1, crc2, b"world!".len() as u64);
assert_eq!(combined, crc32(0, b"Hello, world!"));
```

### `Digest`: Streaming incremental checksum

```rust
use crc32_v2::Digest;

let mut digest = Digest::new();
digest.update(b"Hello, ");
digest.update(b"world!");
assert_eq!(digest.finalize(), 0xEBE6C6E6);

let bytes: [u8; 4] = digest.digest();
assert_eq!(u32::from_be_bytes(bytes), 0xEBE6C6E6);

let existing = crc32_v2::crc32(0, b"prefix:");
let mut d = Digest::with_initial(existing);
d.update(b" suffix");
assert_eq!(d.finalize(), crc32_v2::crc32(existing, b" suffix"));
```

## 📖 API Complexity Table

| Method                        | Time     | Notes                          |
| ----------------------------- | -------- | ------------------------------ |
| `crc32(start, buf)`           | O(n)     | byte-at-a-time, ~343 MiB/s     |
| `crc32_little(start, buf)`    | O(n)     | slicing-by-4, ~833 MiB/s       |
| `crc32_little_8(start, buf)`  | O(n)     | slicing-by-8, ~1 GiB/s         |
| `crc32_little_16(start, buf)` | O(n)     | slicing-by-16, ~1.3 GiB/s      |
| `crc32_big(start, buf)`       | O(n)     | big-endian (unreflected)       |
| `crc32_combine(c1, c2, len2)` | O(log n) | GF(2) repeated squaring        |
| `Digest::new()`               | O(1)     |                                |
| `Digest::update(data)`        | O(n)     | n = data.len()                 |
| `Digest::finalize()`          | O(1)     | does not reset                 |
| `Digest::digest()`            | O(1)     | returns `[u8; 4]` (big-endian) |
| `Digest::reset()`             | O(1)     |                                |

## 🏗 Build-time Code Generation: `crc32-codegen`

The 16 little-endian slicing tables and the big-endian table are generated at
compile-time by the `crc32-codegen` subcrate and written to `$OUT_DIR/crc32tables.rs`.
The main crate includes the generated file via `include!()`.

This pattern (subcrate + `build.rs`) means that the tables are always
consistent with the source, and no generated files are committed to the repository.

## 📊 Running Benchmarks

```sh
# All benchmark groups
cargo bench --bench benchmark
cargo bench --bench nano_benchmark
cargo bench --bench compare_benchmark

# Quick run
cargo bench -- --quick
```

Results are written to `target/criterion/` as interactive HTML reports.
See [BENCHMARKS.md](BENCHMARKS.md) for measured throughput numbers.

## 🔒 Safety

`#![forbid(unsafe_code)]` is enforced crate-wide. All CRC-32 algorithms
(table lookups, slicing loops, GF(2) matrix arithmetic) are pure safe Rust.

## 🔗 See Also

- [docs.rs/crc32-v2](https://docs.rs/crc32-v2)
- [BENCHMARKS.md](BENCHMARKS.md): measured throughput vs zlib-rs, crc32fast, Python
- [PACKAGING.md](PACKAGING.md): Debian and RPM packaging guide
- [PYTHON.md](PYTHON.md): Python bindings guide
- [NODE.md](NODE.md): Node.js bindings guide
- [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
- [zlib - crc32.c](https://github.com/madler/zlib/blob/master/crc32.c)
