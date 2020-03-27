use crate::symbol_table::SymbolTable;
use crate::token::{A, C, Instruction };
use std::fmt::Debug;


fn format_u16(num: &u16) -> String {
    format!("{:016b}", num)
}

pub trait ToBytecode: Debug {
    fn to_bytecode(&self, symbols: &mut SymbolTable) -> Option<String>;
}

impl<'a> ToBytecode for A<'a> {
    fn to_bytecode(&self, table: &mut SymbolTable) -> Option<String> {
        Some(
            match self {
                A::Variable(var) => {
                    match table.get(var) {
                        Some(addr) => format_u16(&addr),
                        None => format_u16(&table.append(var))
                    }
                },
                A::Address(addr) => format_u16(addr)
            }
        )
    }
}

impl<'a> ToBytecode for C<'a> {
    fn to_bytecode(&self, _table: &mut SymbolTable) -> Option<String> {
        let jmp = match self.jump {
            Some(jump) => match jump {
                "JGT" => "001",
                "JEQ" => "010",
                "JGE" => "011",
                "JLT" => "100",
                "JNE" => "101",
                "JLE" => "110",
                "JMP" => "111",
                other => panic!("Unknown jump with {}", other)
            },
            None => "000"
        };

        let dest = match self.dest {
            Some(dest) => match dest {
                "M" =>   "001",
                "D" =>   "010",
                "MD" =>  "011",
                "A" =>   "100",
                "AM" =>  "101",
                "AD" =>  "110",
                "AMD" => "111",
                other => panic!("Unknown dest with {}", other)
            }
            None => "000"
        };

        let comp = match self.comp {
            // a=0
            "0" =>   "0101010",
            "1" =>   "0111111",
            "-1" =>  "0111010",
            "D" =>   "0001100",
            "A" =>   "0110000",
            "!D" =>  "0001101",
            "!A" =>  "0110001",
            "-D" =>  "0001111",
            "-A" =>  "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            // a=1
            "M" =>   "1110000",
            "!M" =>  "1110001",
            "-M" =>  "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => panic!("Unknown C-Instruction {}", self.comp),
        };

        Some(format!("111{}{}{}", comp, dest, jmp))
    }
}

impl<'a> ToBytecode for Instruction<'a> {
    fn to_bytecode(&self, symbols: &mut SymbolTable) -> Option<String> {
        match self {
            Instruction::A(a) => a.to_bytecode(symbols),
            Instruction::C(c) => c.to_bytecode(symbols),
            Instruction::Label(label) => None,
            other => panic!("Unknown token {:?} for bytecode generation", other)
        }
    }
}
