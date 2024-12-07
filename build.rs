use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[path = "src/crc32gen.rs"]
mod crc32gen;

fn main() {
    let outfile_name = "src/crc32tables.rs";

    println!("generating crc tables");

    let crc_tables = crc32gen::make_crc_table();
    let s = crc32gen::write_tables(&crc_tables);

    let outpath = Path::new(outfile_name);

    let outfile = File::create(outpath);

    match outfile {
        Ok(file) => {
            // Wrap the file in a buffered writer
            let mut outwr = io::BufWriter::new(file);

            // Write the generated tables to the file
            if let Err(err) = outwr.write_all(s.as_bytes()) {
                eprintln!("Error writing to file: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Error creating file: {}", err);
        }
    }
}
