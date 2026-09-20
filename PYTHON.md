# CRC32 Python Bindings 🐍

The **`crc32-rs`** package provides blazing-fast CRC-32 functions for Python,
powered by Rust. All functions are **synchronous**: no `asyncio` required.

## 📦 Installation

```sh
pip install crc32-rs
```

Or build locally (requires [maturin](https://github.com/PyO3/maturin)):

```sh
pip install maturin
maturin develop --features python
```

## 🛠 Usage Overview

### One-shot checksum

```python
from crc32_rs import crc32

checksum = crc32(b"Hello, world!")
print(hex(checksum))   # 0xebe6c6e6
```

Chain multiple buffers by passing the previous result:

```python
from crc32_rs import crc32

crc = crc32(b"Hello, ")
crc = crc32(b"world!", crc)
assert crc == crc32(b"Hello, world!")
```

### Slicing-by-4 (higher throughput for large data)

```python
from crc32_rs import crc32_little

checksum = crc32_little(b"Hello, world!")
print(hex(checksum))   # 0xebe6c6e6
```

### Slicing-by-8 (~2.3 GiB/s for large buffers)

```python
from crc32_rs import crc32_little_8

checksum = crc32_little_8(b"Hello, world!")
print(hex(checksum))   # 0xebe6c6e6
```

### Slicing-by-16 (fastest pure-software path, ~3.2 GiB/s)

```python
from crc32_rs import crc32_little_16

data = bytes(range(256)) * 4096
checksum = crc32_little_16(data)
print(hex(checksum))
```

### Big-endian variant

```python
from crc32_rs import crc32_big

checksum = crc32_big(b"Hello, world!")
assert crc32_big(b"") == 0
```

### Streaming checksum with `Digest`

```python
from crc32_rs import Digest

d = Digest()
d.update(b"Hello, ")
d.update(b"world!")
print(hex(d.finalize()))   # 0xebe6c6e6
```

Continue from an existing CRC:

```python
from crc32_rs import crc32, Digest

existing = crc32(b"prefix:")
d = Digest.with_initial(existing)
d.update(b" suffix")
print(hex(d.finalize()))
```

### Combining two checksums

```python
from crc32_rs import crc32, crc32_combine

crc1 = crc32(b"Hello, ")
crc2 = crc32(b"world!")
combined = crc32_combine(crc1, crc2, len(b"world!"))
assert combined == crc32(b"Hello, world!")
```

## API Reference

### Functions

| Function                                      | Description                              |
| --------------------------------------------- | ---------------------------------------- |
| `crc32(data, initial_crc=0) -> int`           | Byte-at-a-time CRC-32 (~350 MiB/s)       |
| `crc32_little(data, initial_crc=0) -> int`    | Slicing-by-4 little-endian (~1.3 GiB/s)  |
| `crc32_little_8(data, initial_crc=0) -> int`  | Slicing-by-8 little-endian (~2.3 GiB/s)  |
| `crc32_little_16(data, initial_crc=0) -> int` | Slicing-by-16 little-endian (~3.2 GiB/s) |
| `crc32_big(data, initial_crc=0) -> int`       | Big-endian (unreflected) CRC-32          |
| `crc32_combine(crc1, crc2, len2) -> int`      | Combine two independent CRC-32 values    |

All functions accept any `bytes`-like object and return an unsigned 32-bit integer.

### `Digest` Class

| Method                     | Description                                     |
| -------------------------- | ----------------------------------------------- |
| `Digest()`                 | Create a new digest starting from CRC `0`       |
| `Digest.with_initial(crc)` | Create a digest continuing from an existing CRC |
| `update(data)`             | Feed bytes into the running checksum            |
| `finalize() -> int`        | Return the current CRC-32 value                 |
| `digest() -> bytes`        | Return 4-byte big-endian representation         |
| `reset()`                  | Reset to CRC `0`                                |

## 📊 Benchmark vs Python zlib

| Method                     | 1 KiB   | 64 KiB | 1 MiB   |
| -------------------------- | ------- | ------ | ------- |
| `crc32_rs.crc32_little_16` | ~300 ns | ~19 µs | ~310 µs |
| `zlib.crc32` (Python)      | ~500 ns | ~32 µs | ~500 µs |

`crc32_little_16` runs approximately **1.5-1.6× faster** than Python's built-in `zlib.crc32` for large buffers.

## 🔗 See Also

- [BENCHMARKS.md](https://github.com/wiseaidev/crc32-v2/blob/main/BENCHMARKS.md): Full benchmark methodology
- [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
- [docs.rs/crc32-v2](https://docs.rs/crc32-v2)
- [PyO3 documentation](https://pyo3.rs)
