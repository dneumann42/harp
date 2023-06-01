use serde_derive::{Deserialize, Serialize};
use self::{
    environment::Env,
    functions::{Call, Exp, Function, Progn},
};

pub mod environment;
pub mod functions;
pub mod intrinsic;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Node {
    Nothing,
    Fun(Function),
    Mod(),
    Intrinsic(String),
    Exp(Exp),
    Call(Call),
    Do(Progn),
}

impl Node {
    pub fn is_fun(&self) -> bool { matches!(self, Node::Fun(_)) }
    pub fn is_mod(&self) -> bool { matches!(self, Node::Mod()) }
    pub fn is_intrinsic(&self) -> bool { matches!(self, Node::Intrinsic(_)) }
    pub fn is_exp(&self) -> bool { matches!(self, Node::Exp(_)) }
    pub fn is_call(&self) -> bool { matches!(self, Node::Call(_)) }
    pub fn is_num(&self) -> bool { matches!(self, Node::Exp(Exp::Num(_))) }
    pub fn is_do(&self) -> bool { matches!(self, Node::Do(_)) }

    pub const fn n(v: f64) -> Self {
        Node::Exp(Exp::Num(v))
    }

    pub fn s<S: ToString>(st: S) -> Node {
        Node::Exp(Exp::Str(st.to_string()))
    }

    pub const fn t() -> Self {
        Self::Exp(Exp::Bol(true))
    }

    pub const fn f() -> Self {
        Self::Exp(Exp::Bol(false))
    }

    pub fn fun(fun: Function) -> Self {
        Self::Fun(fun)
    }

    pub fn arg_list(&self) -> Vec<Node> {
        match self {
            Node::Fun(fun) => {
                match fun.args.as_ref() {
                    Node::Exp(Exp::List(xs)) => {
                        let xs: Vec<Node> = xs.iter().map(|x| x.as_ref().clone()).collect();
                        xs
                    }
                    _ => vec![]
                }
            }
            _ => vec![]
        }
    }

    pub fn a<S: ToString>(s: S) -> Self {
        Self::Exp(Exp::Atom(s.to_string()))
    }

    pub fn get_fun(self) -> Function {
        match self {
            Node::Fun(fun) => fun,
            _ => panic!("Not a function")
        }
    }

    pub fn call_intr<S: Into<String>>(name: S, args: Vec<Node>) -> Node {
        Node::Call(Call::Intrinsic(name.into(), args))
    }

    pub fn call_fun<S: Into<String>>(name: S, args: Vec<Node>) -> Node {
        Node::Call(Call::Fun(name.into(), args))
    }

    pub fn as_num(v: Node) -> f64 {
        v.into()
    }

    pub fn as_bool(b: Node) -> bool {
        match b {
            Node::Nothing => false,
            Node::Exp(Exp::Bol(v)) if !v => false,
            _ => true,
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::Nothing => todo!(),
            Node::Fun(v) => v.to_string(),
            Node::Intrinsic(i) => i.to_string(),
            Node::Exp(e) => e.to_string(),
            Node::Call(c) => c.to_string(),
            Node::Do(progn) => {
                let mut result: Vec<String> = vec![];
                for ele in progn {
                    result.push(ele.to_string())
                }
                result.join(" ")
            }
            Node::Mod() => todo!(),
        }
    }
}

impl From<Exp> for Node {
    fn from(value: Exp) -> Self {
        Node::Exp(value)
    }
}

impl From<Function> for Node {
    fn from(value: Function) -> Self {
        Node::Fun(value)
    }
}

impl From<Call> for Node {
    fn from(value: Call) -> Self {
        Node::Call(value)
    }
}

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Node::n(value)
    }
}

impl From<f64> for Exp {
    fn from(value: f64) -> Self {
        Exp::Num(value)
    }
}

impl From<Exp> for f64 {
    fn from(value: Exp) -> Self {
        match value {
            Exp::Num(v) => v,
            Exp::Bol(v) if v => 1.0,
            _ => 0.0,
        }
    }
}

impl Exp {
    fn as_num(&self) -> f64 {
        match self {
            Exp::Num(n) => *n,
            Exp::Bol(v) if *v => 1.0,
            _ => 0.0,
        }
    }
}

impl From<Node> for f64 {
    fn from(value: Node) -> Self {
        match value {
            Node::Exp(Exp::Bol(b)) if b => 1.0,
            _ => 0.0,
        }
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        if value {
            Node::t()
        } else {
            Node::f()
        }
    }
}

pub type NodeEnv = Env<Node>;

impl NodeEnv {
    pub fn functions(&self) -> Vec<(String, Node)> {
        let mut fs: Vec<(String, Node)> = vec![];
        for scope in self.get_stack() {
            for (s, n) in scope.iter() {
                if n.is_intrinsic() || n.is_fun() {
                    fs.push((s.to_owned(), n.to_owned()));
                }
            }
        }
        fs
    }
}

