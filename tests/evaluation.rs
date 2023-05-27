use harp::{
    evaluator::evaluate,
    nodes::{
        functions::{Call, Exp},
        intrinsic::{Intrinsic, Intrs},
        Node, NodeEnv,
    },
};

const RES1: Node = Node::n(42.0);

struct Test;
impl<'a> Intrinsic<'a> for Test {
    fn name(&self) -> &'static str {
        "test"
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
