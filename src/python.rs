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
//! The bindings provide a **synchronous** API that mirrors the Rust interface.
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
//! >>> from crc32_rs import crc32, crc32_little_16, crc32_hex, crc32_bytes, Digest
//!
//! # One-shot
//! >>> print(hex(crc32(b"Hello, world!")))
//! 0xebe6c6e6
//! >>> print(hex(crc32_little_16(b"Hello, world!")))
//! 0xebe6c6e6
//! >>> print(crc32_hex(b"Hello, world!"))
//! ebe6c6e6
//! >>> print(crc32_bytes(b"Hello, world!").hex())
//! ebe6c6e6
//!
//! # Streaming
//! >>> d = Digest()
//! >>> d.update(b"Hello, ")
//! >>> d.update(b"world!")
//! >>> print(hex(d.finalize()))    
//! 0xebe6c6e6
//! >>> print(d.digest().hex())
//! ebe6c6e6
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
/// >>> from crc32_rs import crc32
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

/// Calculates the CRC-32 checksum and returns it as a 4-byte big-endian value.
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``).
///
/// Returns:
///     The CRC-32 as a 4-byte :class:`bytes` object (big-endian).
///
/// Examples:
///
/// ```python
/// >>> from crc32_rs import crc32_bytes
/// >>> crc32_bytes(b"Hello, world!").hex()
/// 'ebe6c6e6'
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32_bytes<'py>(py: Python<'py>, data: &[u8], initial_crc: u32) -> Bound<'py, PyBytes> {
    let crc = crate::crc32(initial_crc, data);
    PyBytes::new(py, &crc.to_be_bytes())
}

/// Calculates the CRC-32 checksum and returns it as a lowercase hex string.
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``).
///
/// Returns:
///     The CRC-32 as an 8-character lowercase hexadecimal :class:`str`.
///
/// Examples:
///
/// ```python
/// >>> from crc32_rs import crc32_hex
/// >>> crc32_hex(b"Hello, world!")
/// 'ebe6c6e6'
/// >>> crc32_hex(b"123456789")
/// 'cbf43926'
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32_hex(data: &[u8], initial_crc: u32) -> String {
    format!("{:08x}", crate::crc32(initial_crc, data))
}

/// Calculates the CRC-32 checksum using the four-bytes-at-a-time little-endian variant.
///
/// Higher throughput than :func:`crc32` for large buffers (> ~64 bytes).
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``).
///
/// Returns:
///     The CRC-32 as an unsigned 32-bit integer.
///
/// Examples:
///
/// ```python
/// >>> from crc32_rs import crc32_little
/// >>> hex(crc32_little(b"Hello, world!"))
/// '0xebe6c6e6'
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32_little(data: &[u8], initial_crc: u32) -> u32 {
    crate::byfour::crc32_little(initial_crc, data)
}

/// Calculates the CRC-32 checksum using the slicing-by-8 little-endian variant.
///
/// Processes 8 bytes per inner step. Delivers approximately 2× the throughput of
/// :func:`crc32_little` for large buffers.
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``).
///
/// Returns:
///     The CRC-32 as an unsigned 32-bit integer.
///
/// Examples:
///
/// ```python
/// >>> from crc32_rs import crc32_little_8
/// >>> hex(crc32_little_8(b"Hello, world!"))
/// '0xebe6c6e6'
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32_little_8(data: &[u8], initial_crc: u32) -> u32 {
    crate::byfour::crc32_little_8(initial_crc, data)
}

/// Calculates the CRC-32 checksum using the slicing-by-16 little-endian variant.
///
/// The fastest pure-software CRC-32 path in this library. Processes 16 bytes per
/// inner step, delivering approximately 3× the throughput of :func:`crc32_little`.
///
/// Args:
///     data:        Bytes-like object to checksum.
///     initial_crc: Optional initial CRC value (default ``0``).
///
/// Returns:
///     The CRC-32 as an unsigned 32-bit integer.
///
/// Examples:
///
/// ```python
/// >>> from crc32_rs import crc32_little_16
/// >>> hex(crc32_little_16(b"Hello, world!"))
/// '0xebe6c6e6'
/// ```
#[pyfunction]
#[pyo3(signature = (data, initial_crc = 0u32))]
pub fn crc32_little_16(data: &[u8], initial_crc: u32) -> u32 {
    crate::byfour::crc32_little_16(initial_crc, data)
}

/// Calculates the CRC-32 checksum using the big-endian (unreflected) variant.
///
/// Used for hardware CRC devices and UART controllers that operate unreflected.
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
/// >>> from crc32_rs import crc32_big
/// >>> crc32_big(b"") == 0
/// True
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
///     ValueError: If ``len2`` overflows a ``u64``.
///
/// Examples:
///
/// ```python
/// >>> from crc32_rs import crc32, crc32_combine
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
/// Examples:
///
/// ```python
/// >>> from crc32_rs import Digest
/// >>> d = Digest()
/// >>> d.update(b"Hello, ")
/// >>> d.update(b"world!")
/// >>> hex(d.finalize())
/// '0xebe6c6e6'
/// >>> d.digest().hex()
/// 'ebe6c6e6'
/// >>> repr(d)
/// 'Digest(crc=0xEBE6C6E6)'
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

    /// Return the current CRC-32 checksum as an unsigned 32-bit integer.
    ///
    /// Does **not** reset the digest; further :meth:`update` calls continue
    /// from the current state.
    ///
    /// Returns:
    ///     The CRC-32 as an unsigned 32-bit integer.
    pub fn finalize(&self) -> u32 {
        self.inner.finalize()
    }

    /// Return the current CRC-32 as a 4-byte big-endian :class:`bytes` object.
    ///
    /// Equivalent to ``self.finalize().to_bytes(4, 'big')``.
    ///
    /// Returns:
    ///     4-byte :class:`bytes` in big-endian order.
    pub fn digest<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.inner.finalize().to_be_bytes())
    }

    /// Reset the digest to its initial state (CRC ``0``).
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Return a human-readable representation of the digest state.
    ///
    /// Returns:
    ///     ``Digest(crc=0xXXXXXXXX)`` where XXXXXXXX is the current CRC in hex.
    pub fn __repr__(&self) -> String {
        format!("Digest(crc=0x{:08X})", self.inner.finalize())
    }
}

/// Register all Python-exposed types and functions into the `_crc32_v2` module.
///
/// Called from the `#[pymodule]` entry point in `lib.rs`.
pub fn register_python_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crc32, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_hex, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_little, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_little_8, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_little_16, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_big, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_combine, m)?)?;
    m.add_class::<PyDigest>()?;
    Ok(())
}
