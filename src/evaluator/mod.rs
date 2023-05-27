use std::collections::HashSet;

use crate::nodes::{
    environment::Env,
    functions::{Call, Exp},
    intrinsic::Intrs,
    Node, NodeEnv,
};

pub fn evaluate(node: Node, env: &mut NodeEnv, intrs: &Intrs) -> Node {
    match node {
        Node::Exp(exp) => todo!(),
        Node::Call(Call::Intrinsic(name, args)) => {
            let mut evaluated_args: Vec<Exp> = vec![];
            for arg in args {
                match evaluate(arg.into(), env, &intrs) {
                    Node::Exp(ex) => evaluated_args.push(ex),
                    _ => panic!("Expected expression."),
                }
            }
            intrs.matches(&name, env, &evaluated_args)
        }
        Node::Call(Call::Fun(name, args)) => todo!(),
        node => node,
    }
}
