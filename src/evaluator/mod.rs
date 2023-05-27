use std::collections::HashSet;

use crate::nodes::{
    environment::Env,
    functions::{Call, Exp},
    intrinsic::Intrs,
    Node, NodeEnv,
};

pub fn evaluate(node: Node, env: &mut NodeEnv, intrs: &Intrs) -> Node {
    match node {
        Node::Call(Call::Intrinsic(name, args)) => {
            let mut evaluated_args: Vec<Exp> = vec![];
            for arg in args {
                match evaluate(arg.into(), env, &intrs) {
                    Node::Exp(ex) => evaluated_args.push(ex),
                    _ => panic!("Expected expression."),
                }
            }
            match intrs.matches(&name, env, &evaluated_args) {
                Node::Nothing => panic!("Failed to find intrinsic '{}', have you added it?", name),
                v => v,
            }
        }
        Node::Call(Call::Fun(name, args)) => todo!(),
        node => node,
    }
}

pub fn eval_node(node: Node) -> Node {
    evaluate(node, &mut NodeEnv::new(), &Intrs::new().base())
}
