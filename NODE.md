# CRC32 Node.js Bindings 🟩

The **`crc32-rs`** module provides fast, native CRC-32 functions for Node.js,
via [napi-rs](https://napi.rs). All functions are **synchronous**, no Promises required.

## 📦 Installation

```sh
npm install crc32-rs
```

Or build locally:

```sh
npm install -g @napi-rs/cli
napi build --platform --release --features node
```

## 🛠 Usage Overview

### One-shot checksum

```javascript
// If installed via npm: const { crc32 } = require('crc32-rs');
// For local development:
const { crc32 } = require(".");

const checksum = crc32(Buffer.from("Hello, world!"));
console.log(checksum.toString(16)); // ebe6c6e6
```

Chain multiple buffers:

```javascript
// If installed via npm: const { crc32 } = require('crc32-rs');
// For local development:
const { crc32 } = require(".");

let crc = crc32(Buffer.from("Hello, "));
crc = crc32(Buffer.from("world!"), crc);
```

### Slicing-by-4 (higher throughput)

```javascript
// If installed via npm: const { crc32Little } = require('crc32-rs');
// For local development:
const { crc32Little } = require(".");

const checksum = crc32Little(Buffer.from("Hello, world!"));
console.log(checksum.toString(16)); // ebe6c6e6
```

### Slicing-by-8 (~2.3 GiB/s for large buffers)

```javascript
// If installed via npm: const { crc32Little8 } = require('crc32-rs');
// For local development:
const { crc32Little8 } = require(".");

const checksum = crc32Little8(Buffer.from("Hello, world!"));
console.log(checksum.toString(16)); // ebe6c6e6
```

### Slicing-by-16 (fastest pure-software path, ~3.2 GiB/s)

```javascript
// If installed via npm: const { crc32Little16 } = require('crc32-rs');
// For local development:
const { crc32Little16 } = require(".");

const data = Buffer.alloc(1_048_576, 0xab);
const checksum = crc32Little16(data);
console.log(checksum.toString(16));
```

### Big-endian variant

```javascript
// If installed via npm: const { crc32Big } = require('crc32-rs');
// For local development:
const { crc32Big } = require(".");

const checksum = crc32Big(Buffer.from("Hello, world!"));
console.log(checksum.toString(16));
```

### Streaming checksum with `Digest`

```javascript
// If installed via npm: const { Digest } = require('crc32-rs');
// For local development:
const { Digest } = require(".");

const d = new Digest();
d.update(Buffer.from("Hello, "));
d.update(Buffer.from("world!"));
console.log(d.finalize().toString(16)); // ebe6c6e6
```

Continue from an existing CRC:

```javascript
// If installed via npm: const { crc32, Digest } = require('crc32-rs');
// For local development:
const { crc32, Digest } = require(".");

const existing = crc32(Buffer.from("prefix:"));
const d = new Digest(existing);
d.update(Buffer.from(" suffix"));
console.log(d.finalize().toString(16));
```

### Combining two checksums

```javascript
// If installed via npm: const { crc32, crc32Combine } = require('crc32-rs');
// For local development:
const { crc32, crc32Combine } = require(".");

const crc1 = crc32(Buffer.from("Hello, "));
const crc2 = crc32(Buffer.from("world!"));
const combined = crc32Combine(crc1, crc2, Buffer.from("world!").length);
```

## 📖 API Reference

### Functions

| Function                           | Description                              |
| ---------------------------------- | ---------------------------------------- |
| `crc32(data, initialCrc?)`         | Byte-at-a-time CRC-32 (~350 MiB/s)       |
| `crc32Little(data, initialCrc?)`   | Slicing-by-4 little-endian (~1.3 GiB/s)  |
| `crc32Little8(data, initialCrc?)`  | Slicing-by-8 little-endian (~2.3 GiB/s)  |
| `crc32Little16(data, initialCrc?)` | Slicing-by-16 little-endian (~3.2 GiB/s) |
| `crc32Big(data, initialCrc?)`      | Big-endian (unreflected) CRC-32          |
| `crc32Combine(crc1, crc2, len2)`   | Combine two independent CRC-32 values    |

All functions return `number` (unsigned 32-bit integer).

### `Digest` Class

| Method                    | Description                                                 |
| ------------------------- | ----------------------------------------------------------- |
| `new Digest(initialCrc?)` | Create a digest, optionally continuing from an existing CRC |
| `update(data: Buffer)`    | Feed bytes into the running checksum                        |
| `finalize() -> number`    | Return the current CRC-32 value                             |
| `reset()`                 | Reset to CRC `0`                                            |

## 🔗 See Also

- [BENCHMARKS.md](https://github.com/wiseaidev/crc32-v2/blob/main/BENCHMARKS.md): Full benchmark methodology
- [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
- [docs.rs/crc32-v2](https://docs.rs/crc32-v2)
- [napi-rs documentation](https://napi.rs)
