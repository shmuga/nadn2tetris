#[derive(Debug)]
pub struct PushCode<'a> {
    pub segment: &'a str,
    pub i: u32,
}

#[derive(Debug)]
pub struct PopCode<'a> {
    pub segment: &'a str,
    pub i: u32,
}


#[derive(Debug)]
pub enum Opcode<'a> {
    Pop(PopCode<'a>),
    Push(PushCode<'a>),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not
}
