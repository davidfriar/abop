use std::fmt;

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

pub type LString = Vec<Element<ActualParam>>;
pub type Axiom = LString;
pub type Symbol = char;
pub type ActualParam = f32;
pub type FormalParam = char;
pub type Expression = String; //to do

impl<T> ParamList<T>
where
    T: Default + Clone,
{
    pub fn from_vec(vec: &Vec<T>) -> ParamList<T> {
        match vec.len() {
            1 => {
                let mut a: [T; 1] = Default::default();
                a.clone_from_slice(&vec);
                ParamList::A1(a)
            }
            2 => {
                let mut a: [T; 2] = Default::default();
                a.clone_from_slice(&vec);
                ParamList::A2(a)
            }
            3 => {
                let mut a: [T; 3] = Default::default();
                a.clone_from_slice(&vec);
                ParamList::A3(a)
            }
            _ => panic!("too many params"),
        }
    }
}

impl LSystem {
    pub fn new(axiom: Axiom, productions: Vec<Production>) -> Self {
        LSystem {
            current: axiom,
            next: Vec::new(),
            productions: productions,
        }
    }

    pub fn generate(&mut self) -> () {
        for element in self.current.iter() {
            let mut found = false;
            for Production { pred, succ } in self.productions.iter() {
                if pred.matches(&element) {
                    self.next.append(&mut Self::eval(&element, &pred, &succ));
                    found = true;
                    break;
                }
            }
            if !found {
                self.next.push(element.clone());
            }
        }
        std::mem::swap(&mut self.current, &mut self.next);
        self.next.clear();
    }

    pub fn eval(
        _element: &Element<ActualParam>,
        _pred: &Element<FormalParam>,
        succ: &Vec<Element<Expression>>,
    ) -> Vec<Element<ActualParam>> {
        succ.iter()
            .map(|Element { symbol, params }| Element {
                symbol: symbol.clone(),
                params: ParamList::Empty,
            })
            .collect()
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
        for element in self.current.iter() {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}

impl<T> fmt::Display for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol) // to do - display params
    }
}
