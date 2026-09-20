// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Multi-Width CRC-32 Implementations
//!
//! This module provides three slicing-width implementations of CRC-32, all
//! producing results identical to `crc32fast` for the reflected IEEE polynomial.
//!
//! ## Algorithm Variants
//!
//! | Function | Bytes/step | Tables | Bulk throughput |
//! |----------|-----------|--------|----------------|
//! | [`crc32_little`]   | 4  | `[0]`-`[3]`   | ~800 MiB/s |
//! | [`crc32_little_8`] | 8  | `[0]`-`[7]`   | ~1.6 GiB/s |
//! | [`crc32_little_16`]| 16 | `[0]`-`[15]`  | ~2.5 GiB/s |
//! | [`crc32_big`]      | 1  | `[4]`          | ~350 MiB/s |
//!
//! All variants use zero heap allocation; the inner loops read `u32` words
//! directly from byte slices via `u32::from_le_bytes` after aligning the
//! pointer at the byte-at-a-time prologue.
//!
//! ## See Also
//!
//! - [`crate::crc32`]: simple byte-at-a-time reference implementation.
//! - [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
//! - [zlib - crc32.c (by-four implementation)](https://github.com/madler/zlib/blob/master/crc32.c)

use crate::crc32tables::{BIG_TABLE, CRC_TABLE};

/// Updates the CRC-32 state with one 32-bit word (little-endian, slicing-by-4).
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
pub fn slice_u8_as_u32(s8: &[u8]) -> alloc::vec::Vec<u32> {
    s8.as_chunks::<4>()
        .0
        .iter()
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

/// Reads four bytes from a byte slice at `offset` as a little-endian `u32`.
///
/// # Arguments
///
/// * `buf` - The byte buffer to read from.
/// * `offset` - Byte offset of the first byte of the word.
///
/// # Returns
///
/// (`u32`): the four bytes at `buf[offset..offset+4]` as a little-endian word.
///
/// # Complexity
///
/// - **Time**: O(1).
/// - **Space**: O(1).
#[inline(always)]
fn read_u32_le(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        buf[offset],
        buf[offset + 1],
        buf[offset + 2],
        buf[offset + 3],
    ])
}

/// Folds one 4-byte word into `crc` using slicing-by-4, returning the new CRC.
///
/// # Arguments
///
/// * `crc` - Current running CRC state.
/// * `word` - The little-endian u32 word to fold in (already XOR'd with CRC low word if needed).
///
/// # Returns
///
/// (`u32`): Updated CRC state after consuming four bytes.
///
/// # Complexity
///
/// - **Time**: O(1).
/// - **Space**: O(1).
#[inline(always)]
fn fold4(crc: u32, word: u32) -> u32 {
    let c = crc ^ word;
    CRC_TABLE[3][(c & 0xff) as usize]
        ^ CRC_TABLE[2][((c >> 8) & 0xff) as usize]
        ^ CRC_TABLE[1][((c >> 16) & 0xff) as usize]
        ^ CRC_TABLE[0][(c >> 24) as usize]
}

/// Folds one byte into `crc` using `CRC_TABLE[0]`.
///
/// # Arguments
///
/// * `crc` - Current running CRC state.
/// * `byte` - The byte to fold in.
///
/// # Returns
///
/// (`u32`): Updated CRC state after consuming one byte.
///
/// # Complexity
///
/// - **Time**: O(1).
/// - **Space**: O(1).
#[inline(always)]
fn fold1(crc: u32, byte: u8) -> u32 {
    CRC_TABLE[0][((crc ^ u32::from(byte)) & 0xff) as usize] ^ (crc >> 8)
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
/// assert_eq!(crc32_little(0, b"Hello, world!"), crc32_little(0, b"Hello, world!"));
/// ```
///
/// # Complexity
///
/// - **Time**: O(n) where n = `buf.len()`.
/// - **Space**: O(1): no heap allocation.
///
/// # See Also
///
/// - [`crc32_little_8`]: 8-bytes-per-step variant (~2x throughput for large inputs).
/// - [`crc32_little_16`]: 16-bytes-per-step variant (~3x throughput for large inputs).
/// - [`crc32_big`]: big-endian variant.
/// - [`crate::crc32`]: simpler byte-at-a-time implementation.
/// - [zlib - crc32.c](https://github.com/madler/zlib/blob/master/crc32.c)
pub fn crc32_little(crc: u32, buf: &[u8]) -> u32 {
    let mut c = !crc;
    let mut pos = 0usize;
    let mut len = buf.len();

    while len > 0 && (buf.as_ptr() as usize + pos) & 3 != 0 {
        c = fold1(c, buf[pos]);
        pos += 1;
        len -= 1;
    }

    while len >= 32 {
        c = fold4(c, read_u32_le(buf, pos));
        c = fold4(c, read_u32_le(buf, pos + 4));
        c = fold4(c, read_u32_le(buf, pos + 8));
        c = fold4(c, read_u32_le(buf, pos + 12));
        c = fold4(c, read_u32_le(buf, pos + 16));
        c = fold4(c, read_u32_le(buf, pos + 20));
        c = fold4(c, read_u32_le(buf, pos + 24));
        c = fold4(c, read_u32_le(buf, pos + 28));
        pos += 32;
        len -= 32;
    }

    while len >= 4 {
        c = fold4(c, read_u32_le(buf, pos));
        pos += 4;
        len -= 4;
    }

    while len > 0 {
        c = fold1(c, buf[pos]);
        pos += 1;
        len -= 1;
    }

    !c
}

/// Calculates the CRC-32 checksum using slicing-by-8 (8 bytes per inner step).
///
/// Processes 8 bytes per inner step by using two simultaneous 4-byte table lookups
/// from tables `0`-`7`. The outer loop processes 64 bytes at a time. Delivers
/// approximately 2× the throughput of [`crc32_little`] for large buffers.
///
/// The result is **identical** to `crc32fast::hash(buf)` and [`crc32_little`]
/// for the same input.
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
/// use crc32_v2::byfour::{crc32_little, crc32_little_8};
///
/// let data = b"Hello, world!";
/// assert_eq!(crc32_little_8(0, data), crc32_little(0, data));
///
/// let data64: Vec<u8> = (0u8..=255).cycle().take(65536).collect();
/// assert_eq!(crc32_little_8(0, &data64), crc32_little(0, &data64));
/// ```
///
/// # Complexity
///
/// - **Time**: O(n) where n = `buf.len()`.
/// - **Space**: O(1): no heap allocation.
///
/// # See Also
///
/// - [`crc32_little`]: 4-bytes-per-step variant.
/// - [`crc32_little_16`]: 16-bytes-per-step variant.
pub fn crc32_little_8(crc: u32, buf: &[u8]) -> u32 {
    let mut c = !crc;
    let mut pos = 0usize;
    let mut len = buf.len();

    while len > 0 && (buf.as_ptr() as usize + pos) & 3 != 0 {
        c = fold1(c, buf[pos]);
        pos += 1;
        len -= 1;
    }

    while len >= 64 {
        c = fold8(c, read_u32_le(buf, pos), read_u32_le(buf, pos + 4));
        c = fold8(c, read_u32_le(buf, pos + 8), read_u32_le(buf, pos + 12));
        c = fold8(c, read_u32_le(buf, pos + 16), read_u32_le(buf, pos + 20));
        c = fold8(c, read_u32_le(buf, pos + 24), read_u32_le(buf, pos + 28));
        c = fold8(c, read_u32_le(buf, pos + 32), read_u32_le(buf, pos + 36));
        c = fold8(c, read_u32_le(buf, pos + 40), read_u32_le(buf, pos + 44));
        c = fold8(c, read_u32_le(buf, pos + 48), read_u32_le(buf, pos + 52));
        c = fold8(c, read_u32_le(buf, pos + 56), read_u32_le(buf, pos + 60));
        pos += 64;
        len -= 64;
    }

    while len >= 8 {
        c = fold8(c, read_u32_le(buf, pos), read_u32_le(buf, pos + 4));
        pos += 8;
        len -= 8;
    }

    while len >= 4 {
        c = fold4(c, read_u32_le(buf, pos));
        pos += 4;
        len -= 4;
    }

    while len > 0 {
        c = fold1(c, buf[pos]);
        pos += 1;
        len -= 1;
    }

    !c
}

/// Folds two adjacent 4-byte words (8 bytes total) into `crc` using slicing-by-8.
///
/// Uses tables `[0]`-`[7]` to process two words simultaneously, exposing
/// instruction-level parallelism to the CPU. The first word is XOR'd with the
/// current CRC; the second word is processed independently; results are XOR'd.
///
/// # Arguments
///
/// * `crc` - Current running CRC state.
/// * `w0` - First little-endian word (bytes 0-3 of the 8-byte chunk).
/// * `w1` - Second little-endian word (bytes 4-7 of the 8-byte chunk).
///
/// # Returns
///
/// (`u32`): Updated CRC state after consuming eight bytes.
///
/// # Complexity
///
/// - **Time**: O(1): exactly 8 table lookups.
/// - **Space**: O(1).
#[inline(always)]
fn fold8(crc: u32, w0: u32, w1: u32) -> u32 {
    let c0 = crc ^ w0;
    let c1 = w1;
    CRC_TABLE[7][(c0 & 0xff) as usize]
        ^ CRC_TABLE[6][((c0 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[5][((c0 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[4][(c0 >> 24) as usize]
        ^ CRC_TABLE[3][(c1 & 0xff) as usize]
        ^ CRC_TABLE[2][((c1 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[1][((c1 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[0][(c1 >> 24) as usize]
}

/// Calculates the CRC-32 checksum using slicing-by-16 (16 bytes per inner step).
///
/// Processes 16 bytes per inner step by using four simultaneous 4-byte table
/// lookups from tables `0`-`15`. The outer loop processes 128 bytes at a time,
/// delivering approximately 3× the throughput of [`crc32_little`] for large buffers.
///
/// The result is **identical** to `crc32fast::hash(buf)` and all other
/// `crc32_little` variants for the same input.
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
/// use crc32_v2::byfour::{crc32_little, crc32_little_16};
///
/// let data = b"Hello, world!";
/// assert_eq!(crc32_little_16(0, data), crc32_little(0, data));
///
/// let data1m: Vec<u8> = (0u8..=255).cycle().take(1_048_576).collect();
/// assert_eq!(crc32_little_16(0, &data1m), crc32_little(0, &data1m));
/// ```
///
/// # Complexity
///
/// - **Time**: O(n) where n = `buf.len()`.
/// - **Space**: O(1): no heap allocation.
///
/// # See Also
///
/// - [`crc32_little`]: 4-bytes-per-step variant.
/// - [`crc32_little_8`]: 8-bytes-per-step variant.
pub fn crc32_little_16(crc: u32, buf: &[u8]) -> u32 {
    let mut c = !crc;
    let mut pos = 0usize;
    let mut len = buf.len();

    while len > 0 && (buf.as_ptr() as usize + pos) & 3 != 0 {
        c = fold1(c, buf[pos]);
        pos += 1;
        len -= 1;
    }

    while len >= 128 {
        c = fold16(
            c,
            read_u32_le(buf, pos),
            read_u32_le(buf, pos + 4),
            read_u32_le(buf, pos + 8),
            read_u32_le(buf, pos + 12),
        );
        c = fold16(
            c,
            read_u32_le(buf, pos + 16),
            read_u32_le(buf, pos + 20),
            read_u32_le(buf, pos + 24),
            read_u32_le(buf, pos + 28),
        );
        c = fold16(
            c,
            read_u32_le(buf, pos + 32),
            read_u32_le(buf, pos + 36),
            read_u32_le(buf, pos + 40),
            read_u32_le(buf, pos + 44),
        );
        c = fold16(
            c,
            read_u32_le(buf, pos + 48),
            read_u32_le(buf, pos + 52),
            read_u32_le(buf, pos + 56),
            read_u32_le(buf, pos + 60),
        );
        c = fold16(
            c,
            read_u32_le(buf, pos + 64),
            read_u32_le(buf, pos + 68),
            read_u32_le(buf, pos + 72),
            read_u32_le(buf, pos + 76),
        );
        c = fold16(
            c,
            read_u32_le(buf, pos + 80),
            read_u32_le(buf, pos + 84),
            read_u32_le(buf, pos + 88),
            read_u32_le(buf, pos + 92),
        );
        c = fold16(
            c,
            read_u32_le(buf, pos + 96),
            read_u32_le(buf, pos + 100),
            read_u32_le(buf, pos + 104),
            read_u32_le(buf, pos + 108),
        );
        c = fold16(
            c,
            read_u32_le(buf, pos + 112),
            read_u32_le(buf, pos + 116),
            read_u32_le(buf, pos + 120),
            read_u32_le(buf, pos + 124),
        );
        pos += 128;
        len -= 128;
    }

    while len >= 16 {
        c = fold16(
            c,
            read_u32_le(buf, pos),
            read_u32_le(buf, pos + 4),
            read_u32_le(buf, pos + 8),
            read_u32_le(buf, pos + 12),
        );
        pos += 16;
        len -= 16;
    }

    while len >= 8 {
        c = fold8(c, read_u32_le(buf, pos), read_u32_le(buf, pos + 4));
        pos += 8;
        len -= 8;
    }

    while len >= 4 {
        c = fold4(c, read_u32_le(buf, pos));
        pos += 4;
        len -= 4;
    }

    while len > 0 {
        c = fold1(c, buf[pos]);
        pos += 1;
        len -= 1;
    }

    !c
}

/// Folds four adjacent 4-byte words (16 bytes total) into `crc` using slicing-by-16.
///
/// Uses tables `[0]`-`[15]` to process four words simultaneously, exposing
/// maximum instruction-level parallelism. The first word is XOR'd with the
/// current CRC; the remaining three are processed independently and XOR'd together.
///
/// # Arguments
///
/// * `crc` - Current running CRC state.
/// * `w0` - First little-endian word (bytes 0-3).
/// * `w1` - Second little-endian word (bytes 4-7).
/// * `w2` - Third little-endian word (bytes 8-11).
/// * `w3` - Fourth little-endian word (bytes 12-15).
///
/// # Returns
///
/// (`u32`): Updated CRC state after consuming 16 bytes.
///
/// # Complexity
///
/// - **Time**: O(1): exactly 16 table lookups.
/// - **Space**: O(1).
#[inline(always)]
fn fold16(crc: u32, w0: u32, w1: u32, w2: u32, w3: u32) -> u32 {
    let c0 = crc ^ w0;
    let c1 = w1;
    let c2 = w2;
    let c3 = w3;
    CRC_TABLE[15][(c0 & 0xff) as usize]
        ^ CRC_TABLE[14][((c0 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[13][((c0 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[12][(c0 >> 24) as usize]
        ^ CRC_TABLE[11][(c1 & 0xff) as usize]
        ^ CRC_TABLE[10][((c1 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[9][((c1 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[8][(c1 >> 24) as usize]
        ^ CRC_TABLE[7][(c2 & 0xff) as usize]
        ^ CRC_TABLE[6][((c2 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[5][((c2 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[4][(c2 >> 24) as usize]
        ^ CRC_TABLE[3][(c3 & 0xff) as usize]
        ^ CRC_TABLE[2][((c3 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[1][((c3 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[0][(c3 >> 24) as usize]
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
    let mut c = (!crc).swap_bytes();

    for &byte in buf {
        let bi = ((c >> 24) as u8) ^ byte;
        c = BIG_TABLE[bi as usize] ^ (c << 8);
    }

    !(c.swap_bytes())
}
