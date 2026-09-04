// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Python Bindings
//!
//! Exposes the `crc32-v2` library to Python via [`pyo3`].
//! All types and functions are gated behind the `python` Cargo feature.
//!
//! The bindings provide a **synchronous** API that mirrors the Rust interface
//! exactly. No `asyncio` involvement is required.
//!
//! ## Installation
//!
//! Build with [maturin](https://github.com/PyO3/maturin):
//!
//! ```sh
//! pip install maturin
//! maturin develop --features python
//! ```
//!
//! ## Usage
//!
//! ```python
//! from crc32_v2 import crc32, crc32_little, Digest
//!
//! # One-shot
//! print(hex(crc32(b"Hello, world!")))          # 0xebe6c6e6
//! print(hex(crc32_little(b"Hello, world!")))   # 0xebe6c6e6
//!
//! # Streaming
//! d = Digest()
//! d.update(b"Hello, ")
//! d.update(b"world!")
//! print(hex(d.finalize()))   # 0xebe6c6e6
//! ```
//!
//! ## See Also
//!
//! - [A Painless Guide to CRC Error Detection Algorithms](https://www.zlib.net/crc_v3.txt)
//! - [PyO3 documentation](https://pyo3.rs)
//! - [`crate::crc32`]
//! - [`crate::Digest`]

use crate::Digest;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Calculates the CRC-32 checksum of a byte string.
///
/// Compatible with zlib, PKZIP, Ethernet, and FDDI.
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``). Pass a
///                  previous result to chain multiple buffers.
///
/// Returns:
///     The CRC-32 as an unsigned 32-bit integer.
///
/// Raises:
///     ValueError: If ``initial_crc`` is outside the range ``[0, 2^32)``.
///
/// Examples:
///
/// ```python
/// >>> from crc32_v2 import crc32
/// >>> hex(crc32(b"Hello, world!"))
/// '0xebe6c6e6'
/// >>> hex(crc32(b"world!", crc32(b"Hello, ")))
/// '0xebe6c6e6'
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32(data: &[u8], initial_crc: u32) -> u32 {
    crate::crc32(initial_crc, data)
}

/// Calculates the CRC-32 checksum using the four-bytes-at-a-time little-endian variant.
///
/// Higher throughput than :func:`crc32` for large buffers (> ~64 bytes).
/// Produces a different output from :func:`crc32` because the byte processing
/// order differs.
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``).
///
/// Returns:
///     The little-endian CRC-32 as an unsigned 32-bit integer.
///
/// Examples:
///
/// ```python
/// >>> from crc32_v2 import crc32_little
/// >>> hex(crc32_little(b"Hello, world!"))
/// '0xa29eb9bf'
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32_little(data: &[u8], initial_crc: u32) -> u32 {
    crate::byfour::crc32_little(initial_crc, data)
}

/// Calculates the CRC-32 checksum using the big-endian variant.
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``).
///
/// Returns:
///     The big-endian CRC-32 as an unsigned 32-bit integer.
///
/// Examples:
///
/// ```python
/// >>> from crc32_v2 import crc32_big
/// >>> assert crc32_big(b"") == 0
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32_big(data: &[u8], initial_crc: u32) -> u32 {
    crate::byfour::crc32_big(initial_crc, data)
}

/// Combines two CRC-32 values computed over adjacent byte sequences.
///
/// Given ``crc1 = crc32(data1)`` and ``crc2 = crc32(data2)``, returns the
/// CRC-32 of the concatenation ``data1 + data2`` without holding either
/// original byte sequence in memory.
///
/// Args:
///     crc1: CRC-32 of the first sequence.
///     crc2: CRC-32 of the second sequence.
///     len2: Byte length of the second sequence.
///
/// Returns:
///     The CRC-32 of ``data1 + data2`` as an unsigned 32-bit integer.
///
/// Raises:
///     ValueError: If any argument is out of range.
///
/// Examples:
///
/// ```python
/// >>> from crc32_v2 import crc32, crc32_combine
/// >>> c1 = crc32(b"Hello, ")
/// >>> c2 = crc32(b"world!")
/// >>> crc32_combine(c1, c2, len(b"world!")) == crc32(b"Hello, world!")
/// True
/// ```
#[pyfunction]
pub fn crc32_combine(crc1: u32, crc2: u32, len2: u64) -> PyResult<u32> {
    if len2 > u64::MAX / 2 {
        return Err(PyValueError::new_err("len2 is too large"));
    }
    Ok(crate::crc32_combine(crc1, crc2, len2))
}

/// A streaming CRC-32 digest.
///
/// Computes a CRC-32 checksum incrementally over multiple byte buffers.
/// The final result is identical to computing the CRC over the concatenation
/// of all buffers in one shot.
///
/// Construct with :func:`Digest()` (starting from zero) or
/// :func:`Digest.with_initial(initial_crc)` (continuing from an existing CRC).
///
/// All methods (except :meth:`finalize` and :meth:`reset`) return ``self``
/// to allow method chaining.
///
/// Examples:
///
/// ```python
/// >>> from crc32_v2 import Digest
/// >>> d = Digest()
/// >>> d.update(b"Hello, ")
/// >>> d.update(b"world!")
/// >>> hex(d.finalize())
/// '0xebe6c6e6'
/// ```
///
/// See Also:
///     :func:`crc32`: one-shot interface.
///     :func:`crc32_combine`: combine two independently-computed CRCs.
#[pyclass(name = "Digest")]
pub struct PyDigest {
    inner: Digest,
}

#[pymethods]
impl PyDigest {
    /// Create a new :class:`Digest` starting from CRC value ``0``.
    #[new]
    #[pyo3(signature = ())]
    pub fn new() -> Self {
        Self {
            inner: Digest::new(),
        }
    }

    /// Create a :class:`Digest` continuing from an existing CRC value.
    ///
    /// Args:
    ///     initial_crc: A previously computed CRC-32 value.
    ///
    /// Returns:
    ///     A :class:`Digest` pre-loaded with ``initial_crc``.
    #[staticmethod]
    pub fn with_initial(initial_crc: u32) -> Self {
        Self {
            inner: Digest::with_initial(initial_crc),
        }
    }

    /// Feed more bytes into the running checksum.
    ///
    /// Args:
    ///     data: Bytes-like object to incorporate.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Return the current CRC-32 checksum.
    ///
    /// Does **not** reset the digest; further :meth:`update` calls continue
    /// from the current state.
    ///
    /// Returns:
    ///     The CRC-32 as an unsigned 32-bit integer.
    pub fn finalize(&self) -> u32 {
        self.inner.finalize()
    }

    /// Reset the digest to its initial state (CRC ``0``).
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Return a bytes representation of the current CRC-32 (big-endian, 4 bytes).
    pub fn digest<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let value = self.inner.finalize().to_be_bytes();
        PyBytes::new(py, &value)
    }

    pub fn __repr__(&self) -> String {
        format!("Digest(crc=0x{:08X})", self.inner.finalize())
    }
}

/// Register all Python-exposed types and functions into the `_crc32_v2` module.
///
/// Called from the `#[pymodule]` entry point in `lib.rs`.
pub fn register_python_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crc32, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_little, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_big, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_combine, m)?)?;
    m.add_class::<PyDigest>()?;
    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
