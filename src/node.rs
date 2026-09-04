// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Node.js Bindings
//!
//! Exposes the `crc32-v2` library to Node.js via [`napi-derive`].
//! All types and functions are gated behind the `node` Cargo feature.
//!
//! The bindings provide a **synchronous** API. No Promise boilerplate required.
//!
//! ## Installation
//!
//! Build with [napi-rs](https://napi.rs):
//!
//! ```sh
//! npm install -g @napi-rs/cli
//! napi build --platform --release --features node
//! ```
//!
//! ## Usage
//!
//! ```javascript
//! const { crc32, crc32Little, crc32Big, crc32Combine, Digest } = require('.');
//!
//! // One-shot
//! console.log(crc32(Buffer.from('Hello, world!')).toString(16)); // ebe6c6e6
//!
//! // Streaming
//! const d = new Digest();
//! d.update(Buffer.from('Hello, '));
//! d.update(Buffer.from('world!'));
//! console.log(d.finalize().toString(16));   // ebe6c6e6
//! ```
//!
//! ## See Also
//!
//! - [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
//! - [`napi-rs` documentation](https://napi.rs/)
//! - [`crate::crc32`]
//! - [`crate::Digest`]

use crate::Digest;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

/// Calculates the CRC-32 checksum of `data`.
///
/// Compatible with zlib, PKZIP, Ethernet, and FDDI.
///
/// ``data``       : `Buffer` to checksum.
/// ``initialCrc`` : Optional starting CRC value (default ``0``). Pass a
///                   previous result to chain multiple buffers.
///
/// Returns an unsigned 32-bit integer.
///
/// See [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt).
#[napi(js_name = "crc32")]
pub fn napi_crc32(data: Buffer, initial_crc: Option<u32>) -> u32 {
    crate::crc32(initial_crc.unwrap_or(0), &data)
}

/// Calculates the CRC-32 checksum using the four-bytes-at-a-time little-endian variant.
///
/// ``data``       : `Buffer` to checksum.
/// ``initialCrc`` : Optional starting CRC value (default ``0``).
///
/// Returns an unsigned 32-bit integer.
#[napi(js_name = "crc32Little")]
pub fn napi_crc32_little(data: Buffer, initial_crc: Option<u32>) -> u32 {
    crate::byfour::crc32_little(initial_crc.unwrap_or(0), &data)
}

/// Calculates the CRC-32 checksum using the big-endian variant.
///
/// ``data``       : `Buffer` to checksum.
/// ``initialCrc`` : Optional starting CRC value (default ``0``).
///
/// Returns an unsigned 32-bit integer.
#[napi(js_name = "crc32Big")]
pub fn napi_crc32_big(data: Buffer, initial_crc: Option<u32>) -> u32 {
    crate::byfour::crc32_big(initial_crc.unwrap_or(0), &data)
}

/// Combines two CRC-32 values computed over adjacent byte sequences.
///
/// ``crc1``: CRC-32 of the first sequence.
/// ``crc2``: CRC-32 of the second sequence.
/// ``len2``: Byte length of the second sequence.
///
/// Returns the CRC-32 of the concatenation as an unsigned 32-bit integer.
#[napi(js_name = "crc32Combine")]
pub fn napi_crc32_combine(crc1: u32, crc2: u32, len2: i64) -> u32 {
    crate::crc32_combine(crc1, crc2, len2 as u64)
}

/// A streaming CRC-32 digest.
///
/// Computes a CRC-32 checksum incrementally over multiple ``Buffer`` slices.
/// The result is identical to computing the CRC over the concatenation of all
/// buffers in one shot.
///
/// ```javascript
/// const { Digest } = require('.');
/// const d = new Digest();
/// d.update(Buffer.from('Hello, '));
/// d.update(Buffer.from('world!'));
/// console.log(d.finalize().toString(16));  // ebe6c6e6
/// ```
///
/// See Also: ``crc32()``, ``crc32Combine()``.
#[napi(js_name = "Digest")]
pub struct NapiDigest {
    inner: Digest,
}

#[napi]
impl NapiDigest {
    /// Create a new ``Digest`` starting from CRC value ``0``.
    ///
    /// ``initialCrc``: Optional starting CRC value (default ``0``).
    #[napi(constructor)]
    pub fn new(initial_crc: Option<u32>) -> Self {
        let inner = match initial_crc {
            Some(crc) => Digest::with_initial(crc),
            None => Digest::new(),
        };
        Self { inner }
    }

    /// Feed more bytes into the running checksum.
    ///
    /// ``data``: A ``Buffer`` to incorporate into the running CRC.
    #[napi]
    pub fn update(&mut self, data: Buffer) {
        self.inner.update(&data);
    }

    /// Return the current CRC-32 checksum as an unsigned 32-bit integer.
    ///
    /// Does **not** reset the digest; further ``update()`` calls continue
    /// from the current state.
    #[napi]
    pub fn finalize(&self) -> u32 {
        self.inner.finalize()
    }

    /// Reset the digest to CRC ``0``.
    #[napi]
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
