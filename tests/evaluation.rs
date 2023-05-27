use harp::{
    evaluator::{eval_node, evaluate},
    nodes::{
        functions::{Call, Exp},
        intrinsic::{Intrinsic, Intrs},
        Node, NodeEnv,
    },
};

const RES1: Node = Node::n(42.0);

struct Test;
impl<'a> Intrinsic<'a> for Test {
    fn name(&self) -> String {
        "test".to_owned()
    }

    fn call(&self, args: &Vec<Exp>, env: &mut NodeEnv) -> Node {
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
            vec![420.0.into(), Exp::Str("hello".to_string())]
        )),
        Node::s("420 hello")
    )
}
