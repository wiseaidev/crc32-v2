use std::io;
mod crc32gen;

fn main() {

    let args = std::os::args();
    if args.len() != 2 {
        println!("invalid usage.  expected output file path as argument.");
        return;
    }

    let outfile_name = &args[1];

    println!("generating crc tables");

    let crc_tables = crc32gen::make_crc_table();
    let s = crc32gen::write_tables(&crc_tables);

    let outpath = Path::new(outfile_name);

    let outfile = io::File::create(&outpath);
    let mut outwr = io::BufferedWriter::new(outfile);

    outwr.write_str(s.as_slice()).unwrap();
}
