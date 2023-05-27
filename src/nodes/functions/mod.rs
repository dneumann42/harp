#[derive(Clone, Debug)]
pub enum Call {
    Intrinsic(String, Vec<Exp>),
    Fun(String, Vec<Exp>),
}

#[derive(Clone, Debug)]
pub enum Exp {
    Num(f64),
    Atom(String),
    Call(Call),
}

pub type Progn = Vec<Exp>;

#[derive(Clone, Debug)]
pub struct Arg {
    name: String,
}

#[derive(Clone, Debug)]
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
