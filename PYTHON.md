# CRC32 Python Documentation 🐍

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
from crc32_v2 import crc32

checksum = crc32(b"Hello, world!")
print(hex(checksum))   # 0xebe6c6e6
```

Chain multiple buffers by passing the previous result:

```python
from crc32_v2 import crc32

crc = crc32(b"Hello, ")
crc = crc32(b"world!", crc)
assert crc == crc32(b"Hello, world!")
```

### Four-bytes-at-a-time (higher throughput for large data)

```python
from crc32_v2 import crc32_little

checksum = crc32_little(b"Hello, world!")
print(hex(checksum))   # 0xebe6c6e6
```

### Big-endian variant

```python
from crc32_v2 import crc32_big

checksum = crc32_big(b"Hello, world!")
print(hex(checksum)) # 0xebe6c6e6
```

### Streaming checksum with `Digest`

```python
from crc32_v2 import Digest

d = Digest()
d.update(b"Hello, ")
d.update(b"world!")
print(hex(d.finalize()))   # 0xebe6c6e6
```

Continue from an existing CRC:

```python
from crc32_v2 import crc32, Digest

existing = crc32(b"prefix:")
d = Digest.with_initial(existing)
d.update(b" suffix")
print(hex(d.finalize())) # 0x50a75a0d
```

### Combining two checksums

```python
from crc32_v2 import crc32, crc32_combine

crc1 = crc32(b"Hello, ")
crc2 = crc32(b"world!")
combined = crc32_combine(crc1, crc2, len(b"world!"))
assert combined == crc32(b"Hello, world!")
```

## API Reference

### Functions

| Function                                   | Description                               |
| ------------------------------------------ | ----------------------------------------- |
| `crc32(data, initial_crc=0) -> int`        | Standard byte-at-a-time CRC-32            |
| `crc32_little(data, initial_crc=0) -> int` | Four-bytes-at-a-time little-endian CRC-32 |
| `crc32_big(data, initial_crc=0) -> int`    | Four-bytes-at-a-time big-endian CRC-32    |
| `crc32_combine(crc1, crc2, len2) -> int`   | Combine two independent CRC-32 values     |

### `Digest` Class

| Method                     | Description                                     |
| -------------------------- | ----------------------------------------------- |
| `Digest()`                 | Create a new digest starting from CRC `0`       |
| `Digest.with_initial(crc)` | Create a digest continuing from an existing CRC |
| `update(data)`             | Feed bytes into the running checksum            |
| `finalize() -> int`        | Return the current CRC-32 value                 |
| `digest() -> bytes`        | Return 4-byte big-endian representation         |
| `reset()`                  | Reset to CRC `0`                                |

## 🔗 See Also

- [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
- [docs.rs/crc32-v2](https://docs.rs/crc32-v2)
- [PyO3 documentation](https://pyo3.rs)
