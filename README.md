# CRC32

> Fastest CRC32 Rust Implementation

Resurrecting the [`crc32`](https://crates.io/crates/crc32) crate from the ashes.

### Usage

Add `crc32-v2` to your `Cargo.toml` file:

```toml
[dependencies]
crc32-v2 = "0.0.5"
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

### Benchmark

Running `cargo bench` provides the following performance insights:

<details>
<summary><code>cargo bench</code></summary>

```sh
cargo bench

   Compiling crc32-v2 v0.0.5 (/home/mahmoud/Desktop/TODO/crc32-v2)
    Finished `bench` profile [optimized] target(s) in 2.06s
     Running unittests src/lib.rs (target/release/deps/crc32_v2-3c56bd9cac40bc4d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/benchmark.rs (target/release/deps/benchmark-f87f05a33c0b9723)
Benchmarking `crc32-v2` crc32_while_loop(0, b"hello") performance:: Warming up fBenchmarking `crc32-v2` crc32_while_loop(0, b"hello") performance:: Collecting 1`crc32-v2` crc32_while_loop(0, b"hello") performance:
                        time:   [6.6916 ns 6.7111 ns 6.7306 ns]
                        change: [-6.2932% -5.1894% -3.8925%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

Benchmarking `crc32-v2` crc32_for_loop(0, b"hello") performance:: Warming up forBenchmarking `crc32-v2` crc32_for_loop(0, b"hello") performance:: Collecting 100`crc32-v2` crc32_for_loop(0, b"hello") performance:
                        time:   [6.7552 ns 6.7719 ns 6.7884 ns]
                        change: [-16.025% -12.793% -9.5779%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

Benchmarking `crc32fast` crc32fast::hash(b"hello") performance:: Warming up for Benchmarking `crc32fast` crc32fast::hash(b"hello") performance:: Collecting 100 `crc32fast` crc32fast::hash(b"hello") performance:
                        time:   [8.4118 ns 8.4320 ns 8.4522 ns]
                        change: [-9.0567% -7.0970% -5.3253%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

Benchmarking `crc32fast` crc32fast::Hasher::new().update(b"hello").finalize() peBenchmarking `crc32fast` crc32fast::Hasher::new().update(b"hello").finalize() peBenchmarking `crc32fast` crc32fast::Hasher::new().update(b"hello").finalize() peBenchmarking `crc32fast` crc32fast::Hasher::new().update(b"hello").finalize() pe`crc32fast` crc32fast::Hasher::new().update(b"hello").finalize() performance:
                        time:   [21.334 ns 21.379 ns 21.419 ns]
                        change: [-6.8627% -5.2041% -3.7228%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
```

</details>

Below is a summarized table of results, highlighting the execution times for each method:

| **Method**                                     | **Mean Time (ns)** | **Outliers (%)** | **Description**                                |
|-----------------------------------------------|---------------------|-------------------|------------------------------------------------|
| `crc32_v2_while_loop(0, b"hello")`               | 6.71               |    14.00%           | CRC32-V2 built-in function using a while loop, optimized.   |
| `crc32_v2_for_loop(0, b"hello")`                 | 6.77               |    4.00%            | Custom CRC32-V2 using a for loop, optimized.     |
| `crc32fast::hash(b"hello")`                   | 8.43               |    4.00%            | crc32fast built-in hashing method.                |
| `crc32fast::Hasher::new().update(b"hello").finalize()`   | 21.38              |    5.00%            | Hasher object approach, slower initialization.|

1. **Fastest Method**: The built-in `crc32_v2_while_loop` implementation shows the best performance, slightly outperforming the `crc32_for_loop`.
1. **crc32fast Performance**: The `crc32fast::hash` method is slightly slower than the `crc32-v2` implementation.
1. **Hasher Object Method**: The `crc32fast::Hasher` object approach is significantly slower for some reason.
