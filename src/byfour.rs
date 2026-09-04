// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Byte-at-a-Time and Four-Bytes-at-a-Time CRC-32 Implementations
//!
//! This module provides two byte-order-specific implementations of CRC-32
//! that process **four bytes simultaneously** using precomputed lookup tables
//! (`CRC_TABLE[0..=3]` for little-endian, `CRC_TABLE[4..=7]` for big-endian).
//!
//! ## Algorithms
//!
//! | Function | Byte order | Tables used | Best for |
//! |----------|-----------|-------------|----------|
//! | [`crc32_little`] | Little-endian (reflected, LSB first) | `[0]`–`[3]` | Most x86-64 CPUs |
//! | [`crc32_big`] | Big-endian (unreflected, MSB first) | `[4]` | Big-endian hardware CRC |
//!
//! `crc32_little` processes 32 bytes per outer loop iteration and four bytes per
//! inner iteration. `crc32_big` uses a correct byte-at-a-time loop for clarity
//! and correct chaining semantics.
//!
//! ## See Also
//!
//! - [`crate::crc32`]: simple byte-at-a-time reference implementation.
//! - [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
//! - [zlib – crc32.c (by-four implementation)](https://github.com/madler/zlib/blob/master/crc32.c)

use crate::crc32tables::CRC_TABLE;

/// Updates the CRC-32 state with one 32-bit word (little-endian).
///
/// Folds the next four bytes at `buf4[*buf4pos]` into `c` using the
/// slicing-by-4 identity:
///
/// ```text
/// CRC(CRC ⊕ word) = T3[b0] ⊕ T2[b1] ⊕ T1[b2] ⊕ T0[b3]
/// ```
///
/// where `bN` is the N-th byte of `CRC ⊕ word`.
///
/// # Arguments
///
/// * `c` - Running CRC state (XOR-masked).
/// * `buf4` - View of the input buffer cast to `u32` words.
/// * `buf4pos` - Current position in `buf4`, advanced by one on return.
///
/// # Examples
///
/// ```rust
/// use crc32_v2::byfour::dolit4;
///
/// let mut crc = 0u32;
/// let buf = [0u8, 1u8, 2u8, 3u8];
/// let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 1) };
/// let mut pos = 0usize;
/// dolit4(&mut crc, buf4, &mut pos);
/// assert_eq!(crc, 0xAAFD590F);
/// ```
///
/// # Complexity
///
/// - **Time**: O(1): exactly four table lookups.
/// - **Space**: O(1).
///
/// # See Also
///
/// - [`dolit32`]: processes eight words (32 bytes) at once by calling `dolit4` eight times.
/// - [`crc32_little`]
pub fn dolit4(c: &mut u32, buf4: &[u32], buf4pos: &mut usize) {
    let c1 = *c ^ buf4[*buf4pos];
    *buf4pos += 1;
    *c = CRC_TABLE[3][(c1 & 0xff) as usize]
        ^ CRC_TABLE[2][((c1 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[1][((c1 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[0][(c1 >> 24) as usize];
}

/// Updates the CRC-32 state with eight consecutive 32-bit words (32 bytes, little-endian).
///
/// Equivalent to calling [`dolit4`] eight times. Primarily used in the inner
/// loop of [`crc32_little`] to amortise loop overhead over 32 bytes.
///
/// # Arguments
///
/// * `c` - Running CRC state.
/// * `buf4` - View of the input buffer cast to `u32` words.
/// * `buf4pos` - Current position in `buf4`, advanced by eight on return.
///
/// # Examples
///
/// ```rust
/// use crc32_v2::byfour::dolit32;
///
/// let mut crc = 0u32;
/// let buf = [0u8; 32];
/// let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 8) };
/// let mut pos = 0usize;
/// dolit32(&mut crc, buf4, &mut pos);
/// assert_eq!(crc, 0);
/// ```
///
/// # Complexity
///
/// - **Time**: O(1): exactly 32 table lookups.
/// - **Space**: O(1).
///
/// # See Also
///
/// - [`dolit4`]
/// - [`crc32_little`]
pub fn dolit32(c: &mut u32, buf4: &[u32], buf4pos: &mut usize) {
    for _ in 0..8 {
        dolit4(c, buf4, buf4pos);
    }
}

/// Converts a byte slice into a `Vec<u32>` by grouping bytes into little-endian words.
///
/// Trailing bytes that do not form a complete word are discarded. The caller is
/// responsible for handling any residual bytes using the byte-at-a-time path.
///
/// # Arguments
///
/// * `s8` - Byte slice to reinterpret.
///
/// # Returns
///
/// (`Vec<u32>`): little-endian words constructed from complete four-byte groups.
///
/// # Examples
///
/// ```rust
/// use crc32_v2::byfour::slice_u8_as_u32;
///
/// let bytes = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];
/// let words = slice_u8_as_u32(&bytes);
/// assert_eq!(words, &[0x0302_0100u32, 0x0706_0504u32]);
/// ```
///
/// # Complexity
///
/// - **Time**: O(n / 4).
/// - **Space**: O(n / 4).
///
/// # See Also
///
/// - [`crc32_little`]
pub fn slice_u8_as_u32(s8: &[u8]) -> Vec<u32> {
    s8.as_chunks::<4>()
        .0
        .iter()
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

/// Calculates the CRC-32 checksum of `buf` in little-endian (reflected) byte order.
///
/// Processes four bytes per inner iteration (slicing-by-4) and 32 bytes per
/// outer iteration, delivering approximately 4x the throughput of the simple
/// byte-at-a-time [`crate::crc32`] for large buffers (> ~64 bytes). For small
/// buffers the alignment overhead dominates; use [`crate::crc32`] when
/// `buf.len() < 16`.
///
/// The result is **identical** to `crc32fast::hash(buf)` for the same input.
///
/// Pass `crc = 0` to start a fresh computation, or an existing CRC value to
/// chain multiple buffers.
///
/// # Arguments
///
/// * `crc` - Initial or running CRC value. Use `0` for a fresh computation.
/// * `buf` - Byte slice to checksum.
///
/// # Returns
///
/// (`u32`): The CRC-32 of `buf` continuing from `crc`.
///
/// # Examples
///
/// ```rust
/// use crc32_v2::byfour::crc32_little;
///
/// let empty: &[u8] = &[];
/// assert_eq!(crc32_little(0, empty), 0);
/// ```
///
/// # Complexity
///
/// - **Time**: O(n) where n = `buf.len()`.
/// - **Space**: O(n / 4) for the intermediate `u32` slice created by [`slice_u8_as_u32`].
///
/// # See Also
///
/// - [`crc32_big`]: big-endian variant.
/// - [`crate::crc32`]: simpler byte-at-a-time implementation.
/// - [zlib – crc32.c](https://github.com/madler/zlib/blob/master/crc32.c)
pub fn crc32_little(crc: u32, buf: &[u8]) -> u32 {
    let mut len = buf.len();
    let mut bufpos = 0usize;
    let mut c = !crc;

    let mut buf_align_bits = (buf.as_ptr() as usize) & 3;
    while len != 0 && (buf_align_bits & 3) != 0 {
        let bi = (c & 0xff) as u8 ^ buf[bufpos];
        c = CRC_TABLE[0][bi as usize] ^ (c >> 8);
        buf_align_bits += 1;
        bufpos += 1;
        len -= 1;
    }

    let buf4 = slice_u8_as_u32(&buf[bufpos..]);
    let mut buf4pos: usize = 0;

    while len >= 32 {
        dolit32(&mut c, &buf4, &mut buf4pos);
        len -= 32;
    }
    while len >= 4 {
        dolit4(&mut c, &buf4, &mut buf4pos);
        len -= 4;
    }

    bufpos += buf4pos * 4;

    while len > 0 {
        let bi = (c & 0xff) as u8 ^ buf[bufpos];
        c = CRC_TABLE[0][bi as usize] ^ (c >> 8);
        bufpos += 1;
        len -= 1;
    }

    !c
}

/// Calculates the CRC-32 checksum of `buf` in big-endian (unreflected) byte order.
///
/// Uses `CRC_TABLE[4]` (byte-swapped entries) and processes bytes MSB-first.
/// The result differs from [`crc32_little`] because the byte processing
/// order is reversed. This matches hardware CRC calculators on big-endian
/// UART devices that operate unreflected.
///
/// Pass `crc = 0` to start a fresh computation, or an existing CRC value to
/// chain multiple buffers: chaining is fully supported.
///
/// # Arguments
///
/// * `crc` - Initial or running CRC value.
/// * `buf` - Byte slice to checksum.
///
/// # Returns
///
/// (`u32`): The big-endian CRC-32 of `buf` continuing from `crc`.
///
/// # Examples
///
/// ```rust
/// use crc32_v2::byfour::crc32_big;
///
/// let empty: &[u8] = &[];
/// assert_eq!(crc32_big(0, empty), 0);
///
/// let full = crc32_big(0, b"Hello, world!");
/// let chained = crc32_big(crc32_big(0, b"Hello, "), b"world!");
/// assert_eq!(full, chained);
/// ```
///
/// # Complexity
///
/// - **Time**: O(n) where n = `buf.len()`.
/// - **Space**: O(1).
///
/// # See Also
///
/// - [`crc32_little`]: little-endian variant.
/// - [Reflected versus non-reflected CRCs](https://www.zlib.net/crc_v3.txt)
pub fn crc32_big(crc: u32, buf: &[u8]) -> u32 {
    let mut c = crc32gen::zswap32(!crc);

    for &byte in buf {
        let bi = ((c >> 24) as u8) ^ byte;
        c = CRC_TABLE[4][bi as usize] ^ (c << 8);
    }

    !crc32gen::zswap32(c)
}

use crate::crc32gen;
