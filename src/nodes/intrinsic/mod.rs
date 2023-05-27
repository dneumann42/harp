use std::{
    collections::{HashMap, HashSet},
    vec,
};

use super::{
    environment::Env,
    functions::{Arg, Exp},
    Node, NodeEnv,
};

// Intrinsics have no way to evaluate their nodes and so we should
// eval all arguments before calling. We in the future can find a way
// to have them eval so we can make it lazy.

pub trait Intrinsic<'a> {
    fn name(&self) -> String;
    fn call(&self, args: &Vec<Exp>, env: &mut NodeEnv) -> Node;
}

pub struct Intrs<'a> {
    intrs: HashMap<String, Box<dyn Intrinsic<'a>>>,
}

impl<'a> Intrs<'a> {
    pub fn new() -> Self {
        Intrs {
            intrs: HashMap::new(),
        }
    }

    pub fn dup(self) -> Self {
        Self { ..self }
    }

    pub fn intr(self, intr: impl Intrinsic<'a> + 'static) -> Self {
        let mut intrs = self.intrs;
        intrs.insert(intr.name(), Box::new(intr));
        Self { intrs }
    }

    pub fn base(self) -> Self {
        self.intr(Version {}).intr(Add {}).intr(Sub {})
    }

    pub fn matches(&self, name: &str, env: &mut NodeEnv, args: &Vec<Exp>) -> Node {
        self.intrs
            .iter()
            .find(|(_, v)| v.name() == name)
            .map_or(Node::Nothing, |(_, v)| v.call(args, env))
    }
}

impl<'a> Default for Intrs<'a> {
    fn default() -> Self {
        Self {
            intrs: HashMap::new(),
        }
    }
}

struct Version;
struct Add;
struct Sub;
struct Echo;

impl<'a> Intrinsic<'a> for Version {
    fn name(&self) -> String {
        "version".to_owned()
    }

    fn call(&self, _: &Vec<Exp>, _: &mut NodeEnv) -> Node {
        Node::Exp(Exp::Atom("(harp v0.0.0#dev)".to_owned()))
    }
}

impl<'a> Intrinsic<'a> for Add {
    fn name(&self) -> String {
        "+".to_owned()
    }

    fn call(&self, args: &Vec<Exp>, env: &mut NodeEnv) -> Node {
        let mut result = 0.0;
        for arg in args {
            result += arg.as_num();
        }
        result.into()
    }
}

impl<'a> Intrinsic<'a> for Sub {
    fn name(&self) -> String {
        "-".to_owned()
    }

    fn call(&self, args: &Vec<Exp>, env: &mut NodeEnv) -> Node {
        let mut result = args[0].as_num();
        for arg in &args[1..] {
            result -= arg.as_num();
        }
        result.into()
    }
}
