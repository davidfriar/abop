use crate::expr::{Context, Expression};
use rand::prelude::*;
use std::fmt;
use std::iter::FromIterator;

pub type Symbol = char;
pub type ActualParam = f32;
pub type FormalParam = char;

#[derive(Debug)]
pub struct LSystem {
    pub current: LString,
    next: LString,
    productions: Vec<Production>,
    count: u8,
}

#[derive(Debug, Clone)]
pub struct LString(Vec<Element<ActualParam>>);

#[derive(Debug, Clone)]
pub struct Element<T> {
    pub symbol: Symbol,
    pub params: Vec<T>,
}

#[derive(Debug)]
pub struct Production {
    pred: Element<FormalParam>,
    condition: Option<Expression>,
    probability: f32,
    succ: Vec<Element<Expression>>,
}

impl LSystem {
    pub fn new(axiom: LString, productions: Vec<Production>) -> Self {
        LSystem {
            current: axiom,
            next: LString::new(),
            productions,
            count: 0,
        }
    }

    pub fn generate(&mut self) {
        for element in &self.current {
            match self.select_production(&element) {
                Some(production) => {
                    let mut lstring = production.apply(&element);
                    self.next.append(&mut lstring)
                }
                None => self.next.push(element.clone()),
            }
        }
        std::mem::swap(&mut self.current, &mut self.next);
        self.next.clear();
    }

    fn select_production(&self, element: &Element<ActualParam>) -> Option<&Production> {
        let matches: Vec<&Production> = self
            .productions
            .iter()
            .filter(|x| x.matches(&element))
            .collect();
        let r: f32 = rand::thread_rng().gen();
        let mut t: f32 = 0.0;
        for production in matches {
            t += production.probability;
            if r < t {
                return Some(production);
            }
        }
        None
    }
}

impl Iterator for LSystem {
    type Item = LString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.generate();
        }
        self.count += 1;
        Some(self.current.clone())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            self.generate();
        }
        self.next()
    }
}

impl<T> Element<T>
where
    T: Clone + Default,
{
    fn matches<U>(&self, other: &Element<U>) -> bool {
        self.symbol == other.symbol && self.params.len() == other.params.len()
    }

    fn new() -> Element<T> {
        Element {
            symbol: '-',
            params: Vec::new(),
        }
    }
}

impl Production {
    pub fn new() -> Production {
        Production {
            pred: Element::new(),
            condition: None,
            probability: 1.0,
            succ: Vec::new(),
        }
    }

    pub fn set_predecessor(&mut self, pred: Element<FormalParam>) {
        self.pred = pred;
    }

    pub fn set_condition(&mut self, condition: Expression) {
        self.condition = Some(condition);
    }

    pub fn set_probability(&mut self, probability: f32) {
        self.probability = probability;
    }

    pub fn add_successor(&mut self, element: Element<Expression>) {
        self.succ.push(element);
    }

    fn matches(&self, element: &Element<ActualParam>) -> bool {
        self.pred.matches(element)
            && match &self.condition {
                None => true,
                Some(expression) => expression.eval_bool(&self.context(element)),
            }
    }

    fn context(&self, element: &Element<ActualParam>) -> Context {
        self.pred
            .params
            .iter()
            .cloned()
            .zip(element.params.iter().cloned())
            .collect()
    }

    fn apply(&self, element: &Element<ActualParam>) -> LString {
        self.succ
            .iter()
            .map(|Element { symbol, params }| Element {
                symbol: *symbol,
                params: params
                    .into_iter()
                    .map(|param| param.eval(&self.context(element)))
                    .collect(),
            })
            .collect()
    }
}

impl LString {
    pub fn new() -> Self {
        LString(Vec::new())
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn push(&mut self, value: Element<ActualParam>) {
        self.0.push(value);
    }

    pub fn append(&mut self, other: &mut LString) {
        self.0.append(&mut other.0);
    }
}

impl<'a> IntoIterator for &'a LString {
    type Item = &'a Element<ActualParam>;
    type IntoIter = std::slice::Iter<'a, Element<ActualParam>>;

    fn into_iter(self) -> std::slice::Iter<'a, Element<ActualParam>> {
        self.0.iter()
    }
}

impl FromIterator<Element<ActualParam>> for LString {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Element<ActualParam>>,
    {
        LString(Vec::from_iter(iter.into_iter()))
    }
}

impl fmt::Display for LSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.current)?;
        writeln!(f)?;
        for production in &self.productions {
            writeln!(f, "{}", production)?;
        }
        Ok(())
    }
}

impl fmt::Display for LString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for element in self {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)?;
        if self.params.len() > 0 {
            write!(f, "(")?;
            let mut iter = self.params.iter();
            write!(f, "{}", iter.next().unwrap())?;
            for param in iter {
                write!(f, ", {}", param)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}=", self.pred)?;
        for element in &self.succ {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}
