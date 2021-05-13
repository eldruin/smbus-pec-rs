use embedded_crc_macros::{build_rs_lookup_table_file_generation, crc8};

crc8!(fn pec, 7, 0, "");
build_rs_lookup_table_file_generation!(fn write_file, pec, LOOKUP_TABLE, "lookup_table.rs", u8, 256);

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lib.rs");

    write_file().expect("Couldn't write lookup table file!");
}
