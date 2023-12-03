//! # CRC32 V2
//!
//! This crate provides a simple CRC32 implementation in Rust.
//!
//! ## Usage
//!
//! To use this crate, add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! crc32_v2 = "0.0.4"
//! ```
//!
//! Then, you can use the `crc32` or `crc32_little` functions to calculate the CRC32 checksum of a byte buffer.
//!
//! ## Example
//!
//! ```rust
//! use crc32_v2::byfour::crc32_little;
//! use crc32_v2::crc32;
//!
//! let crc = crc32(0, &[0u8, 1u8, 2u8, 3u8]);
//! assert_eq!(crc, 0x8BB98613);
//! let crc_little = crc32_little(crc, &[0u8, 1u8, 2u8, 3u8]);
//! assert_eq!(crc, 0x8BB98613);
//! ```
//!
//! ## Implementation Details
//!
//! The CRC32 algorithm is implemented using a standard polynomial and lookup tables for optimization.
//!
//! The `crc32` function takes two parameters:
//!
//! - `start_crc`: the initial CRC32 value (usually 0)
//! - `buf`: a slice containing the input bytes
//!
//! It returns a `u32`, which is the CRC32 checksum of the input buffer.


pub mod byfour;
pub mod crc32tables;

use crate::crc32tables::CRC_TABLE;

/// This function calculates the CRC32 checksum of a byte buffer using a standard CRC32 algorithm.
///
/// # Arguments
/// * `start_crc` - the initial CRC32 value (usually 0)
/// * `buf` - a slice containing the input bytes
///
/// # Returns
/// (`u32`): the CRC32 checksum of the input buffer
///
/// # Examples
/// ```
/// use crc32_v2::crc32;
///
/// let crc = crc32(0, &[0u8, 1u8, 2u8, 3u8]);
/// assert_eq!(crc, 0x8BB98613);
/// ```
pub fn crc32(start_crc: u32, buf: &[u8]) -> u32 {
    // Initialize variables
    let len = buf.len();
    let mut crc = start_crc;
    let mut bufpos: usize = 0;
    let mut remaining_bytes = len;

    // XOR with 0xffffffff as specified in CRC32 algorithm
    crc = crc ^ 0xffffffff;

    // Reference to the first CRC table for faster access
    let t0 = &CRC_TABLE[0];

    // Process each byte in the buffer
    while remaining_bytes > 0 {
        let b = buf[bufpos];
        let b32 = b as u32;
        let b_index = (crc ^ b32) & 0xff;
        let t = t0[b_index as usize];
        crc = t ^ (crc >> 8);
        bufpos += 1;
        remaining_bytes -= 1;
    }

    // XOR again with 0xffffffff as specified in CRC32 algorithm
    crc ^ 0xffffffff
}
