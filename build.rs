// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[path = "src/crc32gen.rs"]
mod crc32gen;

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/crc32gen.rs");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "node")]
    {
        extern crate napi_build;
        napi_build::setup();
    }

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
    let out_path = Path::new(&out_dir).join("crc32tables.rs");

    let crc_tables = crc32gen::make_crc_table();
    let s = crc32gen::write_tables(&crc_tables);

    let file = File::create(&out_path).expect("could not create crc32tables.rs in OUT_DIR");
    let mut writer = io::BufWriter::new(file);
    writer
        .write_all(s.as_bytes())
        .expect("could not write crc32tables.rs");
}
