use harp::nodes::environment::Env;

#[test]
fn push_and_pop_scopes() {
    use harp::nodes::Node;

    let mut env = Env::<Node>::new();
    env.add("a", Node::Nothing);
    assert!(env.get("a").is_some());
    assert!(env.get("b").is_none());
    env.push();
    env.add("b", Node::Nothing);
    assert!(env.get("a").is_some());
    assert!(env.get("b").is_some());
    assert!(env.get("c").is_none());
    env.pop();
    assert!(env.get("b").is_none());
    assert!(env.get("a").is_some());
}
