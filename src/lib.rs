// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/wiseaidev/crc32-v2/refs/heads/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/wiseaidev/crc32-v2/refs/heads/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod byfour;
pub mod crc32gen;

#[cfg(all(feature = "python", not(feature = "node")))]
pub mod python;

#[cfg(feature = "node")]
pub mod node;

/// Generated CRC-32 lookup tables, produced by `build.rs` at compile time.
pub mod crc32tables {
    include!(concat!(env!("OUT_DIR"), "/crc32tables.rs"));
}

use crc32tables::CRC_TABLE;

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
/// - [`byfour::crc32_little`]: four-bytes-at-a-time variant (higher throughput for large inputs).
/// - [`Digest`]: streaming interface.
/// - [`crc32_combine`]: combine two independently-computed CRC values.
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

/// Combines two CRC-32 values computed over two independent, adjacent byte sequences.
///
/// Given `crc1 = crc32(0, data1)`, `crc2 = crc32(0, data2)`, and
/// `len2 = data2.len()`, returns the CRC-32 of the concatenation
/// `data1 || data2` without requiring either original byte sequence.
///
/// This is the same algorithm used by zlib's `crc32_combine`. It works by
/// multiplying a 32-bit GF(2) state matrix by itself `len2` times using
/// repeated squaring.
///
/// # Arguments
///
/// * `crc1` - CRC-32 of the first byte sequence (`crc32(0, data1)`).
/// * `crc2` - CRC-32 of the second byte sequence (`crc32(0, data2)`).
/// * `len2` - Length in bytes of the second byte sequence.
///
/// # Returns
///
/// (`u32`): The CRC-32 of the concatenated byte sequences.
///
/// # Examples
///
/// ```rust
/// use crc32_v2::{crc32, crc32_combine};
///
/// let crc1 = crc32(0, b"Hello, ");
/// let crc2 = crc32(0, b"world!");
/// let combined = crc32_combine(crc1, crc2, b"world!".len() as u64);
/// assert_eq!(combined, crc32(0, b"Hello, world!"));
/// ```
///
/// # Complexity
///
/// - **Time**: O(log n) where n = `len2` (32 matrix multiplications via repeated squaring).
/// - **Space**: O(1).
///
/// # See Also
///
/// - [`crc32`]
/// - [zlib - crc32_combine](https://github.com/madler/zlib/blob/master/crc32.c)
pub fn crc32_combine(crc1: u32, crc2: u32, len2: u64) -> u32 {
    if len2 == 0 {
        return crc1;
    }

    const GF2_DIM: usize = 32;

    let mut odd: [u32; GF2_DIM] = [0; GF2_DIM];
    let mut even: [u32; GF2_DIM] = [0; GF2_DIM];

    odd[0] = 0xEDB8_8320;
    let mut row: u32 = 1;
    for element in odd.iter_mut().skip(1) {
        *element = row;
        row <<= 1;
    }

    gf2_matrix_square(&mut even, &odd);
    gf2_matrix_square(&mut odd, &even);

    let mut len2 = len2;
    let mut crc1 = crc1;

    loop {
        gf2_matrix_square(&mut even, &odd);
        if (len2 & 1) != 0 {
            crc1 = gf2_matrix_times(&even, crc1);
        }
        len2 >>= 1;
        if len2 == 0 {
            break;
        }
        gf2_matrix_square(&mut odd, &even);
        if (len2 & 1) != 0 {
            crc1 = gf2_matrix_times(&odd, crc1);
        }
        len2 >>= 1;
        if len2 == 0 {
            break;
        }
    }

    crc1 ^ crc2
}

fn gf2_matrix_times(mat: &[u32; 32], mut vec: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut i = 0;

    while vec != 0 {
        if (vec & 1) != 0 {
            sum ^= mat[i];
        }
        vec >>= 1;
        i += 1;
    }

    sum
}

fn gf2_matrix_square(square: &mut [u32; 32], mat: &[u32; 32]) {
    for i in 0..32 {
        square[i] = gf2_matrix_times(mat, mat[i]);
    }
}

/// A streaming CRC-32 digest.
///
/// Allows computing a CRC-32 checksum incrementally over multiple byte slices
/// without buffering the entire input in memory.
///
/// # Examples
///
/// ```rust
/// use crc32_v2::Digest;
///
/// let mut digest = Digest::new();
/// digest.update(b"Hello, ");
/// digest.update(b"world!");
/// assert_eq!(digest.finalize(), 0xEBE6C6E6);
/// ```
///
/// An initial CRC value can be supplied for chaining:
///
/// ```rust
/// use crc32_v2::{crc32, Digest};
///
/// let existing = crc32(0, b"prefix:");
/// let mut digest = Digest::with_initial(existing);
/// digest.update(b" suffix");
/// assert_eq!(digest.finalize(), crc32(existing, b" suffix"));
/// ```
///
/// # See Also
///
/// - [`crc32`]
/// - [`crc32_combine`]
#[derive(Clone, Debug, Default)]
pub struct Digest {
    /// The running CRC state (XOR-masked via the standard `0xFFFFFFFF` convention).
    state: u32,
}

impl Digest {
    /// Creates a new `Digest` starting from CRC value `0`.
    ///
    /// # Returns
    ///
    /// A freshly initialised [`Digest`].
    pub fn new() -> Self {
        Self { state: 0 }
    }

    /// Creates a new `Digest` continuing from an existing CRC value.
    ///
    /// # Arguments
    ///
    /// * `initial_crc` - A previously computed CRC-32 value to continue from.
    ///
    /// # Returns
    ///
    /// A [`Digest`] pre-loaded with `initial_crc`.
    pub fn with_initial(initial_crc: u32) -> Self {
        Self { state: initial_crc }
    }

    /// Feeds more bytes into the running checksum.
    ///
    /// This method may be called any number of times; partial results remain
    /// identical to a single-shot call over the concatenation of all slices.
    ///
    /// # Arguments
    ///
    /// * `data` - The next chunk of bytes to incorporate.
    ///
    /// # Complexity
    ///
    /// - **Time**: O(n) where n = `data.len()`.
    /// - **Space**: O(1).
    pub fn update(&mut self, data: &[u8]) {
        self.state = crc32(self.state, data);
    }

    /// Returns the CRC-32 checksum of all bytes fed so far.
    ///
    /// Calling `finalize` does **not** reset the digest; subsequent
    /// [`update`](Self::update) calls continue from the current state.
    ///
    /// # Returns
    ///
    /// (`u32`): The current CRC-32 checksum.
    ///
    /// # Complexity
    ///
    /// - **Time**: O(1).
    /// - **Space**: O(1).
    pub fn finalize(&self) -> u32 {
        self.state
    }

    /// Resets the digest to its initial state (CRC `0`).
    pub fn reset(&mut self) {
        self.state = 0;
    }
}

#[cfg(all(feature = "python", not(feature = "node")))]
use pyo3::prelude::*;

#[cfg(all(feature = "python", not(feature = "node")))]
#[pymodule]
fn _crc32_v2(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    crate::python::register_python_module(py, m)?;
    Ok(())
}
