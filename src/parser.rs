use crate::config::set_config;
use crate::lsys::{Element, LString, LSystem, Params, Production};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "lsys.pest"]
pub struct LSystemParser;

pub fn parse_lsys(s: &str) -> LSystem {
    let mut axiom: LString = LString::new();
    let mut productions: Vec<Production> = Vec::new();
    LSystemParser::parse(Rule::lsystem, s)
        .expect("Unsuccessful parse")
        .next()
        .unwrap()
        .into_inner()
        .for_each(|r| match r.as_rule() {
            Rule::setting => produce_setting(r),
            Rule::axiom => axiom = produce_axiom(r),
            Rule::production => productions.push(produce_production(r)),
            _ => (),
        });

    LSystem::new(axiom, productions)
}

fn produce_setting(setting: Pair<Rule>) {
    let mut setting = setting.into_inner();
    let name = setting.next().unwrap().as_str();
    let value = setting.next().unwrap().into_inner().next().unwrap();
    match value.as_rule() {
        Rule::number => set_config(name, from_str::<f64>(value)),
        Rule::array => set_config(
            name,
            value
                .into_inner()
                .map(from_str::<f64>)
                .collect::<Vec<f64>>(),
        ),
        _ => unreachable!(),
    };
}

fn produce_axiom(axiom: Pair<Rule>) -> LString {
    axiom.into_inner().map(produce_element).collect()
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
            params: Params::empty(),
        },
    }
}

fn from_str<T: FromStr>(rule: Pair<Rule>) -> T {
    rule.as_str().parse().ok().unwrap()
}
