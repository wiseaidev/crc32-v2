// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CRC-32 Build-Time Table Generator
//!
//! Generates sixteen 256-entry little-endian slicing tables (`CRC_TABLE`) and
//! one 256-entry big-endian table (`BIG_TABLE`) at build time, writing valid
//! Rust source to `$OUT_DIR/crc32tables.rs`.
//!
//! ## Usage
//!
//! Add this crate to `[build-dependencies]` in your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! crc32-codegen = { path = "crc32-codegen", version = "0.1.0" }
//! ```
//!
//! Then call [`run`] from `build.rs`:
//!
//! ```rust,ignore
//! fn main() {
//!     crc32_codegen::run();
//! }
//! ```
//!
//! Include the generated file from your library:
//!
//! ```rust,ignore
//! pub mod crc32tables {
//!     include!(concat!(env!("OUT_DIR"), "/crc32tables.rs"));
//! }
//! ```
//!
//! ## Generated Statics
//!
//! | Static | Type | Description |
//! |--------|------|-------------|
//! | `CRC_TABLE` | `[[u32; 256]; 16]` | 16 little-endian slicing tables |
//! | `BIG_TABLE` | `[u32; 256]` | byte-swapped big-endian table |
//!
//! ## Algorithm
//!
//! The polynomial is the reflected IEEE 802.3 representation `0xEDB88320`.
//! Each additional table `k` satisfies:
//! `T[k][n] = T[0][ T[k-1][n] & 0xFF ] ^ ( T[k-1][n] >> 8 )`
//!
//! ## See Also
//!
//! - [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
//! - [zlib source - crc32.c](https://github.com/madler/zlib/blob/master/crc32.c)

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

/// Number of little-endian slicing tables generated.
///
/// Tables `0`-`3` enable slicing-by-4. Tables `0`-`7` enable slicing-by-8.
/// Tables `0`-`15` enable slicing-by-16.
pub const TBLS: usize = 16;

/// A set of sixteen 256-entry CRC-32 lookup tables (little-endian slicing).
pub type CrcTable = [[u32; 0x100]; TBLS];

/// Reverses (reflects) the byte order of a 32-bit word.
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
/// use crc32_codegen::zswap32;
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
pub fn zswap32(n: u32) -> u32 {
    n.swap_bytes()
}

/// Constructs the sixteen 256-entry little-endian CRC-32 slicing tables and
/// the 256-entry big-endian table.
///
/// Table `0` is the standard byte-at-a-time CRC-32 table for the IEEE 802.3
/// reflected polynomial `0xEDB88320`. Each subsequent table `k` satisfies:
///
/// `T[k][n] = T[0][ T[k-1][n] & 0xFF ] ^ ( T[k-1][n] >> 8 )`
///
/// Returns a tuple of `(CrcTable, [u32; 256])` where the second element is
/// the big-endian table (byte-swapped entries of table `0`).
///
/// # Returns
///
/// `(CrcTable, [u32; 0x100])`: the 16 LE slicing tables and the BE table.
///
/// # Complexity
///
/// - **Time**: O(1): 16 × 256 = 4 096 iterations.
/// - **Space**: O(1): exactly `16 × 256 × 4 = 16 384` bytes.
pub fn make_crc_table() -> (CrcTable, [u32; 0x100]) {
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

    for k in 1..TBLS {
        for n in 0..0x100 {
            let prev = crc_table[k - 1][n];
            crc_table[k][n] = crc_table[0][prev as usize & 0xff] ^ (prev >> 8);
        }
    }

    let mut big_table = [0u32; 0x100];
    for n in 0..0x100 {
        big_table[n] = zswap32(crc_table[0][n]);
    }

    (crc_table, big_table)
}

/// Serialises the 16 LE slicing tables and the BE table to a Rust source string.
///
/// The output is suitable for writing to `OUT_DIR/crc32tables.rs` and
/// including verbatim via:
/// `include!(concat!(env!("OUT_DIR"), "/crc32tables.rs"))`
///
/// # Arguments
///
/// * `crc_table` - The 16 LE slicing tables from [`make_crc_table`].
/// * `big_table` - The 256-entry BE table from [`make_crc_table`].
///
/// # Returns
///
/// (`String`): complete Rust source defining `CRC_TABLE` and `BIG_TABLE`.
///
/// # Complexity
///
/// - **Time**: O(1): always 16 × 256 + 256 = 4 352 entries.
/// - **Space**: O(1): bounded by the fixed table sizes.
pub fn write_tables(crc_table: &CrcTable, big_table: &[u32; 0x100]) -> String {
    let mut s = String::new();

    s.push_str("/* crc32tables.rs -- tables for rapid CRC calculation\n");
    s.push_str(" * Generated automatically by crc32-codegen\n */\n\n");
    s.push_str(&format!(
        "pub static CRC_TABLE: [[u32; 0x100]; {}] = [\n  [\n",
        TBLS
    ));
    write_table_slice(&mut s, &crc_table[0]);

    for item in crc_table.iter().skip(1) {
        s.push_str("  ],\n [\n");
        write_table_slice(&mut s, item);
    }
    s.push_str("  ]\n];\n\n");

    s.push_str("pub static BIG_TABLE: [u32; 0x100] = [\n");
    write_table_slice(&mut s, big_table);
    s.push_str("];\n");

    s
}

/// Serialises a single 256-entry table into Rust literal form.
///
/// Entries are printed five per line as `0x????????`, separated by commas.
///
/// # Arguments
///
/// * `s` - Target string to push the formatted entries into.
/// * `table` - A 256-element slice of `u32` CRC values.
///
/// # Complexity
///
/// - **Time**: O(1): always 256 iterations.
/// - **Space**: O(1): bounded output size per entry.
fn write_table_slice(s: &mut String, table: &[u32; 0x100]) {
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

/// Generates the CRC-32 lookup tables and writes them to `$OUT_DIR/crc32tables.rs`.
///
/// This is the main entry point called from `build.rs`. It performs three steps:
/// 1. Calls [`make_crc_table`] to compute the 16 LE slicing tables and BE table.
/// 2. Calls [`write_tables`] to render them as Rust source.
/// 3. Writes the source to `$OUT_DIR/crc32tables.rs`.
///
/// It also emits `cargo:rerun-if-changed` directives so Cargo re-runs the
/// build script only when relevant source files change.
///
/// # Panics
///
/// Panics if `OUT_DIR` is not set or if writing to the output file fails.
///
/// # Examples
///
/// ```rust,ignore
/// fn main() {
///     crc32_codegen::run();
/// }
/// ```
pub fn run() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=crc32-codegen/src/lib.rs");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
    let out_path = Path::new(&out_dir).join("crc32tables.rs");

    let (crc_tables, big_table) = make_crc_table();
    let s = write_tables(&crc_tables, &big_table);

    let file = File::create(&out_path).expect("could not create crc32tables.rs in OUT_DIR");
    let mut writer = io::BufWriter::new(file);
    writer
        .write_all(s.as_bytes())
        .expect("could not write crc32tables.rs");
}
