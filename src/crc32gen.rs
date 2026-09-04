// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CRC Table Generator
//!
//! Generates the eight 256-entry lookup tables used by the CRC-32 algorithm.
//!
//! The first table (`CRC_TABLE[0]`) is the classic byte-at-a-time table for
//! the IEEE 802.3 polynomial (`0xEDB88320` in reflected form). The remaining
//! seven tables allow the implementation to process four bytes at a time in
//! either little-endian ([`crate::byfour::crc32_little`]) or big-endian
//! ([`crate::byfour::crc32_big`]) byte order.
//!
//! The table is generated at build time by `build.rs`, written to
//! `$OUT_DIR/crc32tables.rs`, and included into the library via a
//! `include!()` macro invocation in `lib.rs`.
//!
//! ## Algorithm
//!
//! The polynomial is
//! `x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1`,
//! whose reflected (LSB-first) representation is `0xEDB88320`.
//!
//! ## See Also
//!
//! - [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
//! - [zlib source - crc32.c](https://github.com/madler/zlib/blob/master/crc32.c)
//! - [IEEE 802.3 CRC-32](https://en.wikipedia.org/wiki/Cyclic_redundancy_check)

/// Number of lookup tables generated.
///
/// Tables `0..=3` are used for little-endian (reflected) four-bytes-at-a-time
/// processing; tables `4..=7` are their byte-swapped counterparts used for
/// big-endian processing.
pub const TBLS: usize = 8;

/// A set of eight 256-entry CRC-32 lookup tables.
///
/// Each `[u32; 0x100]` stores pre-computed CRC contributions for every
/// possible byte value at a specific byte offset within a 32-bit word.
pub type CrcTable = [[u32; 0x100]; TBLS];

/// Constructs the eight 256-entry CRC-32 lookup tables.
///
/// The first table is the standard byte-at-a-time CRC-32 table for the IEEE
/// polynomial. Tables `1`, `2`, and `3` extend it so that four bytes can be
/// folded into the CRC simultaneously (little-endian order). Tables `4`-`7`
/// are byte-swapped versions of tables `0`-`3` for big-endian processing.
///
/// # Returns
///
/// A [`CrcTable`]: eight arrays of 256 `u32` values each.
///
/// # Complexity
///
/// - **Time**: O(1): the loop bounds are compile-time constants (8 x 256 = 2 048 iterations).
/// - **Space**: O(1): the output is always exactly `8 x 256 x 4 = 8 192` bytes.
pub fn make_crc_table() -> CrcTable {
    let p: [u8; 14] = [0, 1, 2, 4, 5, 7, 8, 10, 11, 12, 16, 22, 23, 26];

    let mut poly: u32 = 0;
    for term in p.iter() {
        poly |= 1u32 << (31 - *term as usize);
    }

    let mut crc_table: [[u32; 0x100]; TBLS] = [[0; 0x100]; TBLS];

    for (n, entry) in crc_table[0].iter_mut().enumerate() {
        let mut c = n as u32;
        for _ in 0..8 {
            c = if (c & 1) != 0 {
                poly ^ (c >> 1)
            } else {
                c >> 1
            };
        }
        *entry = c;
    }

    for n in 0..0x100 {
        let mut c: u32 = crc_table[0][n];
        crc_table[4][n] = zswap32(c);
        for k in 1..4 {
            c = crc_table[0][c as usize & 0xff] ^ (c >> 8);
            crc_table[k][n] = c;
            crc_table[k + 4][n] = zswap32(c);
        }
    }

    crc_table
}

/// Reverses (reflects) the byte order of a 32-bit word.
///
/// This is equivalent to `u32::swap_bytes()` and is used to convert
/// little-endian table entries into big-endian table entries.
///
/// # Arguments
///
/// * `n` - The 32-bit value whose bytes are to be swapped.
///
/// # Returns
///
/// (`u32`): bytes of `n` in reversed order.
///
/// # Examples
///
/// ```
/// use crc32_v2::crc32gen::zswap32;
///
/// assert_eq!(zswap32(0x12345678), 0x78563412);
/// assert_eq!(zswap32(0x00000000), 0x00000000);
/// assert_eq!(zswap32(0xFFFFFFFF), 0xFFFFFFFF);
/// ```
///
/// # Complexity
///
/// - **Time**: O(1).
/// - **Space**: O(1).
///
/// # See Also
///
/// - [`u32::swap_bytes`]
pub fn zswap32(n: u32) -> u32 {
    n.swap_bytes()
}

/// Serialises the CRC-32 lookup tables to a Rust source string.
///
/// The output is suitable for writing to `OUT_DIR/crc32tables.rs` and
/// including verbatim via `include!(concat!(env!("OUT_DIR"), "/crc32tables.rs"))`.
///
/// # Arguments
///
/// * `crc_table` - A reference to the table produced by [`make_crc_table`].
///
/// # Returns
///
/// (`String`): the complete Rust source text defining `CRC_TABLE`.
///
/// # Complexity
///
/// - **Time**: O(1): always 8 x 256 = 2 048 entries.
/// - **Space**: O(1): the string length is bounded by the fixed table size.
pub fn write_tables(crc_table: &CrcTable) -> String {
    let mut s = String::new();

    s.push_str("/* crc32tables.rs -- tables for rapid CRC calculation\n");
    s.push_str(" * Generated automatically by crc32gen.rs\n */\n\n");
    s.push_str("pub static CRC_TABLE: [[u32; 0x100]; 8] = [\n  [\n");
    write_table(&mut s, &crc_table[0]);

    s.push_str("// #ifdef BYFOUR\n");
    for item in crc_table.iter().take(8).skip(1) {
        s.push_str("  ],\n [\n");
        write_table(&mut s, item);
    }
    s.push_str("// #endif\n");
    s.push_str("  ]\n];\n");

    s
}

/// Serialises a single 256-entry table into Rust literal form.
///
/// Entries are printed as `0x????????`, five per line, separated by commas.
///
/// # Arguments
///
/// * `s` - Target string to push the formatted entries into.
/// * `table` - A 256-element array of `u32` CRC values.
///
/// # Complexity
///
/// - **Time**: O(1): always 256 iterations.
/// - **Space**: O(1): bounded output size per entry.
fn write_table(s: &mut String, table: &[u32; 0x100]) {
    for (n, item) in table.iter().enumerate().take(0x100) {
        let line = format!(
            "{}0x{:08x}{}",
            if n % 5 != 0 { "" } else { "    " },
            item,
            if n == 255 {
                "\n"
            } else if n % 5 == 4 {
                ",\n"
            } else {
                ", "
            }
        );
        s.push_str(line.as_str());
    }
}
