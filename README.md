# CRC32

Resurrecting the [`crc32`](https://crates.io/crates/crc32) crate from the ashes.

### Usage

Add `crc32-v2` to your `Cargo.toml` file:

```toml
[dependencies]
crc32-v2 = "0.0.3"
```

or run:

```sh
cargo add crc32-v2
```

### Examples

```rust
use crc32_v2::crc32;
use crc32_v2::byfour::crc32_little;

const CRC32_INIT: u32 = 0; // Initial CRC value, you can customize it

fn main() {
    // Your data to calculate CRC for
    let data = b"Hello, world!";

    // Calculate CRC
    let result_crc = crc32(CRC32_INIT, data);

    // Print the result
    println!("CRC-32: {:x}", result_crc);

    // Calculate CRC using the little-endian method
    let result_crc_little = crc32_little(CRC32_INIT, data);

    // Print the result
    println!("CRC-32 (Little Endian): {:x}", result_crc_little);
}

// Output

// CRC-32: ebe6c6e6
// CRC-32 (Little Endian): a29eb9bf
```