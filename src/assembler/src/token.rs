#[derive(Debug)]
pub enum A<'a> {
    Variable(&'a str),
    Address(u16),
}

#[derive(Debug)]
pub struct C<'a> {
    pub dest: Option<&'a str>,
    pub jump: Option<&'a str>,
    pub comp: &'a str,
}

#[derive(Debug)]
pub enum Instruction<'a> {
    A(A<'a>),
    C(C<'a>),
    Label(&'a str)
}
