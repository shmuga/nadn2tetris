extern crate clap;

pub mod opcode;
pub mod parser;
pub mod cli;
pub mod codegen;

use std::fs::{read_to_string, File};
use std::path::Path;
use std::io::Write;

pub fn translate(filepath: &str, source: &str) -> String {
    let filename = Path::new(&filepath)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("NoFilename")
        .to_string();

    let opcodes = parser::parse(source);
    codegen::generate(&opcodes, filename)
}

fn main() {
    let args = cli::args();
    read_to_string(&args.input)
        .map(|file| translate(&args.input, &file))
        .and_then(|output| File::create(args.output)
            .and_then(|mut f| f.write_all(output.as_bytes())))
        .map_err(|_| "Could not write result");
}
