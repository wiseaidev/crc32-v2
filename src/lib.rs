// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), doc = "")]
#![cfg_attr(feature = "std", doc = include_str!("../README.md"))]
#![cfg_attr(feature = "std", doc = include_str!("../RUST.md"))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/wiseaidev/crc32-v2/refs/heads/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/wiseaidev/crc32-v2/refs/heads/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(any(feature = "node", not(feature = "std")), allow(unsafe_code))]
#![cfg_attr(not(any(feature = "node", not(feature = "std"))), forbid(unsafe_code))]

extern crate alloc;

pub mod byfour;
pub mod combine;
pub mod digest;
pub mod tables;

#[cfg(all(feature = "python", not(feature = "node"), feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "python", feature = "std"))))]
pub mod python;

#[cfg(all(feature = "node", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "node", feature = "std"))))]
pub mod node;

/// Generated CRC-32 lookup tables, produced by `crc32-codegen` at build time.
pub mod crc32tables {
    include!(concat!(env!("OUT_DIR"), "/crc32tables.rs"));
}

pub use combine::crc32_combine;
pub use digest::Digest;
pub use tables::crc32;

#[cfg(all(feature = "python", not(feature = "node"), feature = "std"))]
use pyo3::prelude::*;

#[cfg(all(feature = "python", not(feature = "node"), feature = "std"))]
#[pymodule]
fn _crc32_v2(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    crate::python::register_python_module(py, m)?;
    Ok(())
}

#[cfg(all(not(feature = "std"), not(test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(feature = "std"), not(test)))]
#[global_allocator]
static ALLOCATOR: DummyAlloc = DummyAlloc;

#[cfg(all(not(feature = "std"), not(test)))]
struct DummyAlloc;

#[cfg(all(not(feature = "std"), not(test)))]
unsafe impl core::alloc::GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}
