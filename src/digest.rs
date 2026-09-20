// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Streaming CRC-32 Digest
//!
//! Provides the [`Digest`] type: an incremental CRC-32 accumulator that
//! accepts multiple byte slices and produces the same result as a single-shot
//! call over their concatenation.

use crate::tables::crc32;

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
/// - [`crate::crc32`]
/// - [`crate::crc32_combine`]
#[derive(Clone, Debug, Default)]
pub struct Digest {
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

    /// Returns the CRC-32 as a 4-byte big-endian representation.
    ///
    /// Equivalent to `finalize().to_be_bytes()`.
    ///
    /// # Returns
    ///
    /// (`[u8; 4]`): big-endian bytes of the current checksum.
    ///
    /// # Complexity
    ///
    /// - **Time**: O(1).
    /// - **Space**: O(1).
    pub fn digest(&self) -> [u8; 4] {
        self.state.to_be_bytes()
    }

    /// Resets the digest to its initial state (CRC `0`).
    pub fn reset(&mut self) {
        self.state = 0;
    }
}
