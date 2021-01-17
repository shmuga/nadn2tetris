extern crate clap;

pub mod cli;
pub mod codegen;
pub mod opcode;
pub mod parser;

use std::fs::{read_dir, read_to_string, File};
use std::io::{Error, Write};
use std::path::Path;

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

    let entries = read_dir(&args.input)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()
        .unwrap();

    let entries = entries
        .iter()
        .filter(|path| {
            path.extension()
                .map(|ext| ext.to_str().unwrap_or("") == "vm")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let entry = if entries.len() == 1 {
        "".to_string()
    } else {
        codegen::entry()
    };

    let mut output = entries
        .iter()
        .fold(entry, |mut acc, x| {
            let single_file = read_to_string(x).unwrap();
            let assembly = translate(&x.to_str().unwrap_or("<noname>"), &single_file);

            acc.push_str(&assembly);
            acc
        });

    // output.push_str(&codegen::entry());

    File::create(args.output)
        .and_then(|mut f| f.write_all(output.as_bytes()))
        .unwrap();
}
