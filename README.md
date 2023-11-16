# CRC32

Resurrecting the [`crc32`](https://crates.io/crates/crc32) crate from the ashes.

### Usage

Add `crc32-v2` to your `Cargo.toml` file:

```toml
[dependencies]
crc = "3.0"
```

or run:

```sh
cargo add crc32-v2
```

### Examples

```rust
use crc32_v2::crc32;
use crc32_v2::byfour::{crc32_little, dolit32};

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

    // Example for using dolit32
    let buf4: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut c = CRC32_INIT;
    let mut buf4pos = 0;
    dolit32(&mut c, &buf4, &mut buf4pos);

    // Print the result
    println!("CRC-32 after dolit32: {:x}", c);
}
```

> **Warning**<br>
The `dolit4` and `dolit32` functions modify the input `c` and `buf4pos` in place, so you should be careful when reusing these variables for subsequent CRC calculations.
