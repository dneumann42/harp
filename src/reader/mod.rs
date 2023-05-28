use std::vec;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::nodes::{functions::Exp, Node};

#[derive(Parser)]
#[grammar = "reader/grammar.pest"]
pub struct HarpParser;

#[derive(Debug, Clone)]
pub enum ParseErr {
    Never,
    Eof,
    ExpectedAtom(String),
    ParseNumberError(String),
}

type Result<T> = std::result::Result<T, ParseErr>;

fn unblock(node: Node) -> Node {
    match node.clone() {
        Node::Do(ndo) => {
            if ndo.len() > 0 {
                match &ndo[0] {
                    Node::Do(xs) => Node::Do(xs.to_owned()),
                    _ => node,
                }
            } else {
                node
            }
        }
        v => v,
    }
}

pub fn read<S: ToString>(code: S) -> Result<Node> {
    match HarpParser::parse(Rule::script, code.to_string().as_str()) {
        Ok(rs) => parse_rules(rs).map(unblock),
        Err(_e) => {
            todo!()
        }
    }
}

pub fn parse_rules(rs: Pairs<Rule>) -> Result<Node> {
    rs.map(parse_rule)
        .collect::<Result<Vec<Node>>>()
        .map(Node::Do)
}

pub fn parse_rule(r: Pair<Rule>) -> Result<Node> {
    let x = match r.as_rule() {
        Rule::EOI => Err(ParseErr::Eof),
        Rule::script => {
            let mut ns: Vec<Node> = vec![];
            for sub in r.into_inner() {
                if let Rule::EOI = sub.as_rule() {
                    continue;
                }
                ns.push(parse_rule(sub)?);
            }
            Ok(Node::Do(ns))
        }
        Rule::WHITESPACE => todo!(),
        Rule::alpha => todo!(),
        Rule::digit => todo!(),
        Rule::underscore => todo!(),
        Rule::list => {
            let mut ns: Vec<Node> = vec![];
            for sub in r.into_inner() {
                if let Rule::EOI = sub.as_rule() {
                    continue;
                }
                ns.push(parse_rule(sub)?);
            }

            if ns.len() == 0 {
                return Ok(Node::Exp(Exp::List(vec![])));
            }

            match &ns[0] {
                Node::Exp(Exp::Atom(_)) => {}
                _ => {
                    return Err(ParseErr::ExpectedAtom(format!(
                        "Error: expected atom to call but got '{:?}'",
                        &ns[0]
                    )))
                }
            }

            Ok(Node::call_intr(
                ns[0].to_string(),
                ns[1..].iter().map(|n| n.to_owned()).collect::<Vec<Node>>(),
            ))
        }
        Rule::array => todo!(),
        Rule::exp => {
            for x in r.into_inner() {
                return parse_rule(x);
            }
            Err(ParseErr::Never)
        }
        Rule::boolean => match r.as_str().to_lowercase().as_str() {
            "#t" => Ok(Node::t()),
            "#f" => Ok(Node::f()),
            _ => Err(ParseErr::Never),
        },
        Rule::char => todo!(),
        Rule::atom => Ok(Node::Exp(Exp::Atom(r.as_str().to_owned()))),
        Rule::number => {
            return r
                .as_str()
                .parse::<f64>()
                .map_err(|err| ParseErr::ParseNumberError(err.to_string()))
                .map(|v| {
                    let x = v.into();
                    return x;
                })
        }
        Rule::string => todo!(),
    };
    x
}
