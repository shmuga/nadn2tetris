use std::collections::HashMap;
use crate::token::Instruction;
use std::process::id;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    last_used: u16,
    map: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbols = SymbolTable {
            map: HashMap::new(),
            last_used: 15
        };

        symbols.init();
        symbols
    }

    fn init(&mut self) {
        self.insert("SP", 0);
        self.insert("LCL", 1);
        self.insert("ARG", 2);
        self.insert("THIS", 3);
        self.insert("THAT", 4);
        self.insert("SCREEN", 16384);
        self.insert("KBD", 24576);

        self.insert("R0", 0);
        self.insert("R1", 1);
        self.insert("R2", 2);
        self.insert("R3", 3);
        self.insert("R4", 4);
        self.insert("R5", 5);
        self.insert("R6", 6);
        self.insert("R7", 7);
        self.insert("R8", 8);
        self.insert("R9", 9);
        self.insert("R10", 10);
        self.insert("R11", 11);
        self.insert("R12", 12);
        self.insert("R13", 13);
        self.insert("R14", 14);
        self.insert("R15", 15);
    }

    pub fn insert(&mut self, symbol: &str, address: u16) {
        self.map.insert(String::from(symbol), address);
    }

    pub fn append(&mut self, symbol: &str) -> u16 {
        match self.map.get(symbol) {
            None => {
                self.last_used += 1;
                self.map.insert(String::from(symbol), self.last_used);
                self.last_used
            }
            Some(n) => n.clone()
        }
    }

    pub fn get(&self, symbol: &str) -> Option<u16> {
        self.map.get(symbol).cloned()
    }
}

