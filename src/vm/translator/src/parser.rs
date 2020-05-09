use crate::opcode::*;

fn clean_line<'a>(line: &'a str) -> &'a str {
    line.trim()
        .splitn(2, "/")
        .next()
        .unwrap_or("")
        .trim()
}

pub fn parse_stack<'a>(line: &'a str) -> Opcode<'a> {
    let words: Vec<&str> = line.split(" ").collect();

    match words[..] {
        ["push", segment, i] => Opcode::Push(PushCode { segment, i: i.parse().unwrap_or(0) }),
        ["pop", segment, i] => Opcode::Pop(PopCode { segment, i: i.parse().unwrap_or(0) }),
        _ => panic!(format!("Unknown expression: {}", line))
    }

}

pub fn parse(source: &str) -> Vec<Opcode> {
    source
        .lines()
        .map(clean_line)
        .filter(|line| line.len() > 0)
        .map(|line| match line {
            "add" => Opcode::Add,
            "sub" => Opcode::Sub,
            "neg" => Opcode::Neg,
            "eq" => Opcode::Eq,
            "gt" => Opcode::Gt,
            "lt" => Opcode::Lt,
            "and" => Opcode::And,
            "or" => Opcode::Or,
            "not" => Opcode::Not,
            _ => parse_stack(&line)
        })
        .collect()
}
