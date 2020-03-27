extern crate clap;

use std::fs::{read_to_string, File};
use std::intrinsics::write_bytes;
use std::io::Write;

pub mod lexer;
pub mod token;
pub mod cli;
pub mod symbol_table;
pub mod interpreter;
pub mod to_bytecode;

pub fn assemble(source: &str) -> String {
    let mut table = symbol_table::SymbolTable::new();
    let lexer = lexer::Lexer::new(&source, &mut table);
    let res = interpreter::to_machine_code(lexer.table, &lexer.tokens);
    res
}

fn main() {
    let args = cli::args();
    read_to_string(&args.input)
        .map(|file| assemble(&file))
        .and_then(|output| File::create(args.output)
            .and_then(|mut f| f.write_all(output.as_bytes())))
        .map_err(|e| "Could not write result");
}

