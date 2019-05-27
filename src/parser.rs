use crate::lsys::{Axiom, Element, LSystem, ParamList, Production};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "lsys.pest"]
pub struct LSystemParser;

pub fn parse_lsys(s: &str) -> LSystem {
    let mut lsys = LSystemParser::parse(Rule::lsystem, s)
        .expect("Unsuccessful parse")
        .next()
        .unwrap()
        .into_inner();
    LSystem::new(
        produce_axiom(lsys.next().unwrap()),
        produce_productions(lsys),
    )
}

fn produce_axiom(axiom: Pair<Rule>) -> Axiom {
    axiom.into_inner().map(produce_element).collect()
}

fn produce_productions(productions: Pairs<Rule>) -> Vec<Production> {
    productions
        .take_while(|r| r.as_rule() == Rule::production)
        .map(produce_production)
        .collect()
}

fn produce_production(production: Pair<Rule>) -> Production {
    let mut production = production.into_inner();
    Production {
        pred: produce_element(production.next().unwrap()),
        succ: production.map(produce_element).collect(),
    }
}

fn produce_element<T>(element: Pair<Rule>) -> Element<T>
where
    T: FromStr + Clone + Default,
{
    let mut element = element.into_inner();
    let symbol = from_str(element.next().unwrap());
    match element.next() {
        Some(params) => Element {
            symbol,
            params: params.into_inner().map(from_str).collect(),
        },
        None => Element {
            symbol,
            params: ParamList::Empty,
        },
    }
}

fn from_str<T: FromStr>(rule: Pair<Rule>) -> T {
    rule.as_str().parse().ok().unwrap()
}
