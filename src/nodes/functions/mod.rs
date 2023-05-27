#[derive(Clone, Debug, PartialEq)]
pub enum Call {
    Intrinsic(String, Vec<Exp>),
    Fun(String, Vec<Exp>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp {
    Nothing,
    Num(f64),
    Bol(bool),
    Atom(String),
    Call(Call),
}

pub type Progn = Vec<Exp>;

#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: String,
    args: Vec<Arg>,
    body: Progn,
}

impl Function {
    pub fn new(name: String, args: Vec<Arg>, body: Progn) -> Self {
        Self { name, args, body }
    }
}
