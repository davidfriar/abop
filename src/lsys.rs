use crate::expr::{Context, Expression};
use std::fmt;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct LSystem {
    pub current: LString,
    next: LString,
    productions: Vec<Production>,
}

#[derive(Debug, Clone)]
pub struct Element<T> {
    pub symbol: Symbol,
    pub params: ParamList<T>,
}

#[derive(Debug, Clone)]
pub enum ParamList<T> {
    Empty,
    A1([T; 1]),
    A2([T; 2]),
    A3([T; 3]),
}

#[derive(Debug)]
pub struct Production {
    pub pred: Element<FormalParam>,
    pub succ: Vec<Element<Expression>>,
}

impl Production {
    fn matches(&self, element: &Element<ActualParam>) -> bool {
        self.pred.matches(element)
    }

    fn apply(&self, element: &Element<ActualParam>) -> LString {
        let context: Context = self
            .pred
            .params
            .into_iter()
            .cloned()
            .zip(element.params.into_iter().cloned())
            .collect();

        self.succ
            .iter()
            .map(|Element { symbol, params }| Element {
                symbol: *symbol,
                params: params
                    .into_iter()
                    .map(|param| param.eval(&context))
                    .collect(),
            })
            .collect()
    }
}

pub type LString = Vec<Element<ActualParam>>;
pub type Axiom = LString;
pub type Symbol = char;
pub type ActualParam = f32;
pub type FormalParam = char;

impl<T> ParamList<T>
where
    T: Default + Clone,
{
    pub fn from_slice(slice: &[T]) -> ParamList<T> {
        match slice.len() {
            0 => ParamList::Empty,
            1 => {
                let mut a: [T; 1] = Default::default();
                a.clone_from_slice(&slice);
                ParamList::A1(a)
            }
            2 => {
                let mut a: [T; 2] = Default::default();
                a.clone_from_slice(&slice);
                ParamList::A2(a)
            }
            3 => {
                let mut a: [T; 3] = Default::default();
                a.clone_from_slice(&slice);
                ParamList::A3(a)
            }
            _ => panic!("too many params"),
        }
    }
}

impl<'a, T> IntoIterator for &'a ParamList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            ParamList::Empty => [].iter(),
            ParamList::A1(a) => a.iter(),
            ParamList::A2(a) => a.iter(),
            ParamList::A3(a) => a.iter(),
        }
    }
}

impl<T> FromIterator<T> for ParamList<T>
where
    T: Default + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let v: Vec<T> = iter.into_iter().collect();
        ParamList::from_slice(&v[..])
    }
}

impl LSystem {
    pub fn new(axiom: Axiom, productions: Vec<Production>) -> Self {
        LSystem {
            current: axiom,
            next: Vec::new(),
            productions,
        }
    }

    pub fn generate(&mut self) {
        for element in &self.current {
            match self.select_production(element) {
                Some(production) => self.next.append(&mut production.apply(&element)),
                None => self.next.push(element.clone()),
            }
        }
        std::mem::swap(&mut self.current, &mut self.next);
        self.next.clear();
    }

    fn select_production(&self, element: &Element<ActualParam>) -> Option<&Production> {
        self.productions.iter().find(|x| x.matches(&element))
    }
}

impl Iterator for LSystem {
    type Item = LString;

    fn next(&mut self) -> Option<Self::Item> {
        self.generate();
        Some(self.current.clone())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            self.generate();
        }
        self.next()
    }
}

impl<T> Element<T> {
    fn matches<U>(&self, other: &Element<U>) -> bool {
        self.symbol == other.symbol && self.params.same_arity(&other.params)
    }
}

impl<T> ParamList<T> {
    fn same_arity<U>(&self, other: &ParamList<U>) -> bool {
        match (self, other) {
            (ParamList::Empty, ParamList::Empty) => true,
            (ParamList::A1(_), ParamList::A1(_)) => true,
            (ParamList::A2(_), ParamList::A2(_)) => true,
            (ParamList::A3(_), ParamList::A3(_)) => true,
            (_, _) => false,
        }
    }
}

impl fmt::Display for LSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for element in &self.current {
            write!(f, "{}", element)?;
        }
        writeln!(f)?;
        for production in &self.productions {
            writeln!(f, "{}", production)?;
        }
        Ok(())
    }
}
impl<T: fmt::Display> fmt::Display for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.symbol, self.params)
    }
}

impl<T: fmt::Display> fmt::Display for ParamList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamList::Empty => Ok(()),
            ParamList::A1(a) => write!(f, "({})", a[0]),
            ParamList::A2(a) => write!(f, "({}, {})", a[0], a[1]),
            ParamList::A3(a) => write!(f, "({},{},{})", a[0], a[1], a[2]),
        }
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
