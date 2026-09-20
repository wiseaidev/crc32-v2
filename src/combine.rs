// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # GF(2) Matrix Arithmetic and CRC Combination
//!
//! Provides [`crc32_combine`] and the internal GF(2) matrix helpers used to
//! combine two independently-computed CRC-32 values without the original data.

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
/// - [`crate::crc32`]
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

/// Multiplies a GF(2) matrix by a vector (32-bit CRC state).
///
/// Iterates over each set bit in `vec`, XOR-ing in the corresponding row
/// of `mat`. This implements the binary polynomial multiply-by-vector
/// operation over GF(2).
///
/// # Arguments
///
/// * `mat` - The 32-element GF(2) matrix (each `u32` is a row of 32 bits).
/// * `vec` - The 32-bit vector to multiply.
///
/// # Returns
///
/// (`u32`): The product of `mat × vec` in GF(2).
///
/// # Complexity
///
/// - **Time**: O(32) = O(1).
/// - **Space**: O(1).
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

/// Squares a 32×32 GF(2) matrix in-place.
///
/// Computes `square[i] = mat × mat[i]` for all `i`, effectively computing
/// the matrix product `mat × mat`. Used by [`crc32_combine`] to implement
/// repeated squaring over the CRC polynomial.
///
/// # Arguments
///
/// * `square` - The output matrix; overwritten with `mat²`.
/// * `mat` - The input matrix to square.
///
/// # Complexity
///
/// - **Time**: O(32²) = O(1).
/// - **Space**: O(1).
fn gf2_matrix_square(square: &mut [u32; 32], mat: &[u32; 32]) {
    for i in 0..32 {
        square[i] = gf2_matrix_times(mat, mat[i]);
    }
}
