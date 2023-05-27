use std::collections::binary_heap::Iter;

use self::{
    environment::Env,
    functions::{Call, Exp, Function},
    intrinsic::Intrinsic,
};

pub mod environment;
pub mod functions;
pub mod intrinsic;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Nothing,
    Fun(Function),
    Mod(),
    Intrinsic(String),
    Exp(Exp),
    Call(Call),
}

pub type NodeEnv = Env<Node>;

impl From<Exp> for Node {
    fn from(value: Exp) -> Self {
        Node::Exp(value)
    }
}

impl From<Function> for Node {
    fn from(value: Function) -> Self {
        Node::Fun(value)
    }
}

impl From<Call> for Node {
    fn from(value: Call) -> Self {
        Node::Call(value)
    }
}

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Node::n(value)
    }
}

impl From<f64> for Exp {
    fn from(value: f64) -> Self {
        Exp::Num(value)
    }
}

impl From<Exp> for f64 {
    fn from(value: Exp) -> Self {
        match value {
            Exp::Num(v) => v,
            Exp::Bol(v) if v => 1.0,
            _ => 0.0,
        }
    }
}

impl Exp {
    fn as_num(&self) -> f64 {
        match self {
            Exp::Num(n) => *n,
            Exp::Bol(v) if *v => 1.0,
            _ => 0.0,
        }
    }
}

impl From<Node> for f64 {
    fn from(value: Node) -> Self {
        match value {
            Node::Exp(Exp::Bol(b)) if b => 1.0,
            _ => 0.0,
        }
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        if value {
            Node::t()
        } else {
            Node::f()
        }
    }
}

impl Node {
    pub const fn n(v: f64) -> Self {
        Node::Exp(Exp::Num(v))
    }

    pub const fn t() -> Self {
        Self::Exp(Exp::Bol(true))
    }

    pub const fn f() -> Self {
        Self::Exp(Exp::Bol(false))
    }

    pub fn call_intr<S: Into<String>>(name: S, args: Vec<Exp>) -> Node {
        Node::Call(Call::Intrinsic(name.into(), args))
    }

    pub fn as_num(v: Node) -> f64 {
        v.into()
    }

    pub fn as_bool(b: Node) -> bool {
        match b {
            Node::Nothing => false,
            Node::Exp(Exp::Bol(v)) if !v => false,
            _ => true,
        }
    }
}
