use crate::token::{Instruction, A, C};
use crate::token::Instruction::Label;
use crate::symbol_table::SymbolTable;

#[derive(Debug)]
pub struct Lexer<'a> {
    counter: u16,
    pub table: &'a mut SymbolTable,
    pub tokens: Vec<Instruction<'a>>,
}

fn is_digit(x: &Option<char>) -> bool {
    x.map(|x| x == '_' || '0' <= x && x <= '9').unwrap_or(false)
}

impl <'a> Lexer<'a> {
    pub fn new(source: &'a str, symbols: &'a mut SymbolTable) -> Lexer<'a> {
        let mut p = Lexer {
            tokens: Vec::new(),
            counter: 0,
            table: symbols,
        };

        p.tokenize(source);
        p
    }

    fn clean_line(&self, line: &'a str) -> &'a str {
        line.trim()
            .splitn(2, "/")
            .next()
            .unwrap_or("")
            .trim()
    }

    fn match_a(&mut self, line: &'a str) -> A<'a> {
        self.counter += 1;
        let trimmed = line.trim_start_matches("@");

        match trimmed.parse::<u16>() {
            Ok(addr) => A::Address(addr),
            Err(_) => A::Variable(trimmed),
        }
    }

    fn match_c(&mut self, line: &'a str) -> C<'a> {
        self.counter += 1;
        if line.contains("=") {
            let (dest, comp) = line.split_at(line.find("=").unwrap());
            C { jump: None, dest: Some(dest), comp: comp.trim_start_matches("=") }
        } else {
            let (comp, jmp) = line.split_at(line.find(";").unwrap());
            C { jump: Some(jmp.trim_start_matches(";")), dest: None, comp }
        }
    }

    fn match_label(&mut self, line: &'a str) -> &'a str {
        let label = line.trim_start_matches("(").trim_end_matches(")");
        self.table.insert(&label, self.counter);
        label
    }

    fn tokenize(&mut self, source: &'a str) {
        for line in source.split("\n") {
            let clean = self.clean_line(line);

            if clean.len() == 0 {
                continue;
            }

            let token = match clean.chars().next() {
                Some('@') => Instruction::A(self.match_a(&clean)),
                Some('(') => Instruction::Label(self.match_label(clean)),
                Some(_) => Instruction::C(self.match_c(&clean)),
                None => panic!("Can't find 0 char for line {}", line)
            };

            self.tokens.push(token);
        }

    }
}

