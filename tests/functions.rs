use harp::nodes::{environment::Env, functions::Function, Node};

#[test]
fn that_we_can_define_functions() {
    let mut env = Env::<Node>::new();

    env.add("+", Node::Intrinsic("+".to_string()));

    let _fun = Function::new("".to_owned(), vec![], vec![]);

    assert!(true)
}
