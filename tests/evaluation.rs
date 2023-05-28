use harp::{
    evaluator::{eval_node, evaluate},
    nodes::{
        functions::Exp,
        intrinsic::{Intrinsic, Intrs},
        Node, NodeEnv,
    },
    reader::read,
};

const RES1: Node = Node::n(42.0);

struct Test;
impl<'a> Intrinsic<'a> for Test {
    fn name(&self) -> String {
        "test".to_owned()
    }

    fn call(&self, _args: &Vec<Exp>, _env: &mut NodeEnv) -> Node {
        RES1
    }
}

#[test]
fn that_we_can_create_intrinsics_outside_package() {
    assert_eq!(
        evaluate(
            Node::call_intr("test", vec![]),
            &mut NodeEnv::new(),
            &Intrs::new().base().intr(Test {}),
        ),
        RES1
    );
}

#[test]
fn that_we_can_add_and_subtract() {
    assert_eq!(
        evaluate(
            Node::call_intr("+", vec![100.0.into(), 200.0.into()]),
            &mut NodeEnv::new(),
            &Intrs::new().base(),
        ),
        300.0.into()
    );

    assert_eq!(
        evaluate(
            Node::call_intr("-", vec![300.0.into(), 200.0.into()]),
            &mut NodeEnv::new(),
            &Intrs::new().base(),
        ),
        100.0.into()
    )
}

#[test]
fn that_we_can_print_things() {
    assert_eq!(
        eval_node(Node::call_intr(
            "print",
            vec![420.0.into(), Node::s("hello")]
        )),
        Node::s("420 hello")
    )
}

#[test]
fn that_we_can_nest_expressions() {
    assert_eq!(
        evaluate(
            Node::call_intr(
                "+",
                vec![
                    100.0.into(),
                    Node::call_intr("-", vec![Node::n(42.0), Node::n(22.0)]),
                    200.0.into()
                ]
            ),
            &mut NodeEnv::new(),
            &Intrs::new().base(),
        ),
        (320.0).into()
    );
}

#[test]
fn that_add_sub() {
    let exp = read("(+ 1 (- 7 5) 3)").unwrap();
    let res = evaluate(exp, &mut NodeEnv::new(), &Intrs::new().base());
    assert_eq!(res, Node::n(6.0));
}
