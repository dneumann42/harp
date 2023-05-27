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

impl Node {
    pub const fn n(v: f64) -> Self {
        Node::Exp(Exp::Num(v))
    }

    pub fn call_intr<S: Into<String>>(name: S, args: Vec<Exp>) -> Node {
        Node::Call(Call::Intrinsic(name.into(), args))
    }

    pub fn as_num(v: Node) -> f64 {
        match v {
            Node::Exp(Exp::Num(n)) => n,
            Node::Exp(Exp::Bol(b)) if b => 1.0,
            _ => 0.0,
        }
    }

    pub fn as_bool(b: Node) -> bool {
        match b {
            Node::Nothing => false,
            Node::Exp(Exp::Bol(v)) if !v => false,
            _ => true,
        }
    }
}
