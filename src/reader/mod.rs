use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::nodes::{functions::Exp, Node};

#[derive(Parser)]
#[grammar = "reader/grammar.pest"]
pub struct HarpParser;

struct Ast {
    node: Node,
    charindex: i32,
}

#[derive(Debug, Clone)]
pub enum ParseErr {}

type Result<T> = std::result::Result<T, ParseErr>;

pub fn read<S: ToString>(code: S) -> Result<Node> {
    match HarpParser::parse(Rule::script, code.to_string().as_str()) {
        Ok(rs) => parse_rules(rs),
        Err(e) => todo!(),
    }
}

pub fn parse_rules(rs: Pairs<Rule>) -> Result<Node> {
    rs.map(parse_rule)
        .collect::<Result<Vec<Exp>>>()
        .map(|rs| Node::Do(rs))
}

pub fn parse_rule(r: Pair<Rule>) -> Result<Exp> {
    match r.as_rule() {
        Rule::EOI => todo!(),
        Rule::script => todo!(),
        Rule::WHITESPACE => todo!(),
        Rule::alpha => todo!(),
        Rule::digit => todo!(),
        Rule::underscore => todo!(),
        Rule::list => todo!(),
        Rule::array => todo!(),
        Rule::expr => todo!(),
        Rule::boolean => todo!(),
        Rule::char => todo!(),
        Rule::atom => todo!(),
        Rule::number => todo!(),
        Rule::string => todo!(),
    }
}
