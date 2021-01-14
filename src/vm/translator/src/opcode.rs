#[derive(Debug)]
pub struct SegmentMetadata<'a> {
    pub segment: &'a str,
    pub i: u32,
}

#[derive(Debug)]
pub struct LabelMetadata<'a> {
    pub name: &'a str,
}

#[derive(Debug)]
pub struct FunctionMetadata<'a> {
    pub name: &'a str,
    pub argv: u32,
}


#[derive(Debug)]
pub enum Opcode<'a> {
    Call(FunctionMetadata<'a>),
    Function(FunctionMetadata<'a>),
    Return,

    Goto(LabelMetadata<'a>),
    IfGoto(LabelMetadata<'a>),
    Label(LabelMetadata<'a>),

    Pop(SegmentMetadata<'a>),
    Push(SegmentMetadata<'a>),

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
