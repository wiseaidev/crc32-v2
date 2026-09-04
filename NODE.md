# CRC32 Node.js Documentation 🟩

The **`crc32-rs`** module provides fast, native CRC-32 functions for Node.js,
via [napi-rs](https://napi.rs). All functions are **synchronous** - no Promises required.

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

Enter the node interpreter:

```sh
node
```

### One-shot checksum

```javascript
// If installed via npm: const { crc32 } = require('crc32-rs');
// For local development:
const { crc32 } = require(".");

const checksum = crc32(Buffer.from("Hello, world!"));
console.log(checksum.toString(16)); // ebe6c6e6
```

Chain multiple buffers by passing the previous result:

```javascript
// If installed via npm: const { crc32 } = require('crc32-rs');
// For local development:
const { crc32 } = require(".");

let crc = crc32(Buffer.from("Hello, "));
crc = crc32(Buffer.from("world!"), crc);
```

### Four-bytes-at-a-time (higher throughput for large data)

```javascript
// If installed via npm: const { crc32Little } = require('crc32-rs');
// For local development:
const { crc32Little } = require(".");

const checksum = crc32Little(Buffer.from("Hello, world!"));
console.log(checksum.toString(16)); // ebe6c6e6
```

### Big-endian variant

```javascript
// If installed via npm: const { crc32Big } = require('crc32-rs');
// For local development:
const { crc32Big } = require(".");

const checksum = crc32Big(Buffer.from("Hello, world!"));
console.log(checksum.toString(16)); // ebe6c6e6
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
console.log(d.finalize().toString(16)); // 50a75a0d
```

### Combining two checksums

```javascript
// If installed via npm: const { crc32, crc32Combine } = require('crc32-rs');
// For local development:
const { crc32, crc32Combine } = require(".");

const crc1 = crc32(Buffer.from("Hello, "));
const crc2 = crc32(Buffer.from("world!"));
const combined = crc32Combine(crc1, crc2, Buffer.from("world!").length); // 3957769958
```

## 📖 API Reference

### Functions

| Function                         | Description                               |
| -------------------------------- | ----------------------------------------- |
| `crc32(data, initialCrc?)`       | Standard byte-at-a-time CRC-32            |
| `crc32Little(data, initialCrc?)` | Four-bytes-at-a-time little-endian CRC-32 |
| `crc32Big(data, initialCrc?)`    | Four-bytes-at-a-time big-endian CRC-32    |
| `crc32Combine(crc1, crc2, len2)` | Combine two independent CRC-32 values     |

All functions return `number` (unsigned 32-bit integer).

### `Digest` Class

| Method                    | Description                                                 |
| ------------------------- | ----------------------------------------------------------- |
| `new Digest(initialCrc?)` | Create a digest, optionally continuing from an existing CRC |
| `update(data: Buffer)`    | Feed bytes into the running checksum                        |
| `finalize() -> number`    | Return the current CRC-32 value                             |
| `reset()`                 | Reset to CRC `0`                                            |

## 🔗 See Also

- [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
- [docs.rs/crc32-v2](https://docs.rs/crc32-v2)
- [napi-rs documentation](https://napi.rs)
