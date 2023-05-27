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
