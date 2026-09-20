// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CRC-32 Byte-at-a-Time Table Lookup
//!
//! Provides the fundamental `crc32` byte-at-a-time function using the reflected
//! IEEE 802.3 polynomial `0xEDB88320`. The lookup table is generated at build
//! time by the `crc32-codegen` subcrate and included via `include!()`.

use crate::crc32tables::CRC_TABLE;

/// Calculates the CRC-32 checksum of `buf`, continuing from `start_crc`.
///
/// Uses the reflected IEEE 802.3 polynomial (`0xEDB88320`) and a standard
/// byte-at-a-time table lookup: compatible with zlib, PKZIP, Ethernet,
/// and FDDI.
///
/// Pass `start_crc = 0` to start a fresh computation. Pass the result of a
/// previous call as `start_crc` to chain over multiple buffers:
///
/// ```rust
/// use crc32_v2::crc32;
///
/// let crc = crc32(crc32(0, b"Hello, "), b"world!");
/// assert_eq!(crc, crc32(0, b"Hello, world!"));
/// ```
///
/// # Arguments
///
/// * `start_crc` - The initial or running CRC value. Use `0` for a fresh computation.
/// * `buf` - A slice of bytes to checksum.
///
/// # Returns
///
/// (`u32`): The CRC-32 of `buf` continuing from `start_crc`.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `buf.len()`.
/// - **Space**: O(1).
///
/// # See Also
///
/// - [`crate::byfour::crc32_little`]: four-bytes-at-a-time variant.
/// - [`crate::byfour::crc32_little_8`]: eight-bytes-at-a-time variant.
/// - [`crate::byfour::crc32_little_16`]: sixteen-bytes-at-a-time variant (highest throughput).
/// - [`crate::Digest`]: streaming interface.
/// - [`crate::crc32_combine`]: combine two independently-computed CRC values.
/// - [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
#[inline]
pub fn crc32(start_crc: u32, buf: &[u8]) -> u32 {
    let mut crc = start_crc ^ 0xffff_ffff;

    for &byte in buf {
        let index = (crc ^ u32::from(byte)) & 0xff;
        crc = CRC_TABLE[0][index as usize] ^ (crc >> 8);
    }

    crc ^ 0xffff_ffff
}
