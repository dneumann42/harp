use crate::nodes::{
    functions::{Call, Exp},
    intrinsic::Intrs,
    Node, NodeEnv,
};

pub fn evaluate_do(xs: Vec<Node>, env: &mut NodeEnv, intrs: &Intrs) -> Node {
    xs.iter()
        .map(|x| evaluate(x.clone(), env, intrs))
        .last()
        .unwrap_or(Node::Nothing)
}

pub fn evaluate(node: Node, env: &mut NodeEnv, intrs: &Intrs) -> Node {
    match node {
        Node::Do(xs) => evaluate_do(xs, env, intrs),
        Node::Call(Call::Intrinsic(name, args)) => {
            let evaluated_args: Vec<Exp> = args
                .into_iter()
                .map(|arg| match evaluate(arg.into(), env, &intrs) {
                    Node::Exp(ex) => ex,
                    _ => panic!("Expected expression."),
                })
                .collect();

            match intrs.matches(&name, env, &evaluated_args) {
                Node::Nothing => panic!("Failed to find intrinsic '{}', have you added it?", name),
                v => v,
            }
        }
        Node::Exp(Exp::Call(call)) => evaluate(Node::Call(call), env, intrs),
        node => node,
    }
}

pub fn eval_node(node: Node) -> Node {
    evaluate(node, &mut NodeEnv::new(), &Intrs::new().base())
}
