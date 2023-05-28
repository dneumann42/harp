use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use super::Node;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Call {
    Intrinsic(String, Vec<Node>),
    Fun(String, Vec<Node>),
}

impl ToString for Call {
    fn to_string(&self) -> String {
        match self {
            Call::Intrinsic(name, args) | Call::Fun(name, args) => format!(
                "({} {})",
                name,
                args.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Exp {
    Nothing,
    Num(f64),
    Bol(bool),
    Atom(String),
    Str(String),
    List(Vec<Box<Node>>),
    Dict(HashMap<String, Node>),
    Call(Call),
}

impl Exp {
    pub fn call_intr<S: Into<String>>(name: S, args: Vec<Node>) -> Exp {
        let x = name.into();
        Exp::Call(Call::Intrinsic(x, args))
    }
}

impl ToString for Exp {
    fn to_string(&self) -> String {
        match self {
            Exp::Nothing => "nothing".to_owned(),
            Exp::Num(v) => v.to_string(),
            Exp::Bol(b) => b.to_string(),
            Exp::Atom(a) => a.to_string(),
            Exp::Call(c) => c.to_string(),
            Exp::Str(s) => s.to_owned(),
            Exp::List(xs) => {
                format!(
                    "({})",
                    xs.iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Exp::Dict(xs) => {
                format!("#({})",
                        xs.iter()
                            .map(|(k, v)| format!("{} {}", k, v.to_string()))
                            .collect::<Vec<String>>()
                            .join("  "))
            }
        }
    }
}

pub type Progn = Vec<Node>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Arg {
    name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Function {
    name: String,
    args: Vec<Arg>,
    body: Progn,
}

impl Function {
    pub fn new(name: String, args: Vec<Arg>, body: Progn) -> Self {
        Self { name, args, body }
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        format!(
            "<fun:{} {}>",
            self.name,
            self.args
                .iter()
                .map(|e| e.name.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
