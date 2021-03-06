use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use std::str::FromStr;

/// Represents an simple mathematical expression
/// which can be evaluated in a context in order to obtain a result
/// ```
/// use abop::expr::{Expression, Context};
/// let exp : Expression = "1".parse().ok().unwrap();
/// let mut context: Context = Default::default();
/// assert_eq!(exp.eval(&context), 1.0);
/// context.push(('x', 4.0));
/// context.push(('y', 6.0));
/// let exp : Expression = "(x^2+6*2)*(10-y)".parse().ok().unwrap();
/// assert_eq!(exp.eval(&context), 112.0);
/// ```
#[derive(Debug, Clone)]
pub enum Expression {
    Var(Var),
    Value(Value),
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GE(Box<Expression>, Box<Expression>),
    LE(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
}

type Var = char;
type Value = f32;

pub type Context = Vec<(Var, Value)>;

impl Expression {
    pub fn eval(&self, context: &Context) -> Value {
        match self {
            Expression::Value(x) => *x,
            Expression::Var(x) => Self::lookup(context, *x),

            Expression::Or(x, y) => {
                Self::as_value(Self::as_bool(x.eval(context)) || Self::as_bool(y.eval(context)))
            }
            Expression::And(x, y) => {
                Self::as_value(Self::as_bool(x.eval(context)) && Self::as_bool(y.eval(context)))
            }
            Expression::Eq(x, y) => Self::as_value(x.eval(context) == y.eval(context)),
            Expression::GT(x, y) => Self::as_value(x.eval(context) > y.eval(context)),
            Expression::LT(x, y) => Self::as_value(x.eval(context) < y.eval(context)),
            Expression::GE(x, y) => Self::as_value(x.eval(context) >= y.eval(context)),
            Expression::LE(x, y) => Self::as_value(x.eval(context) <= y.eval(context)),
            Expression::Add(x, y) => x.eval(context) + y.eval(context),
            Expression::Sub(x, y) => x.eval(context) - y.eval(context),
            Expression::Mul(x, y) => x.eval(context) * y.eval(context),
            Expression::Div(x, y) => x.eval(context) / y.eval(context),
            Expression::Pow(x, y) => x.eval(context).powf(y.eval(context)),
        }
    }

    pub fn eval_bool(&self, context: &Context) -> bool {
        Self::as_bool(self.eval(context))
    }

    fn as_bool(x: Value) -> bool {
        !(x == 0.0)
    }

    fn as_value(x: bool) -> Value {
        if x {
            1.0
        } else {
            0.0
        }
    }

    fn lookup(context: &Context, var: char) -> Value {
        context.iter().find(|(x, _)| *x == var).unwrap().1 // to do : error handling
    }

    fn build_expression(expression: Pairs<Rule>) -> Expression {
        PREC_CLIMBER.climb(
            expression,
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::number => Expression::Value(pair.as_str().parse::<f32>().unwrap()),
                Rule::expr => Self::build_expression(pair.into_inner()),
                Rule::var => Expression::Var(pair.as_str().parse::<char>().unwrap()),
                _ => unreachable!(),
            },
            |lhs: Expression, op: Pair<Rule>, rhs: Expression| match op.as_rule() {
                Rule::or => Expression::Or(Box::new(lhs), Box::new(rhs)),
                Rule::and => Expression::And(Box::new(lhs), Box::new(rhs)),
                Rule::eq => Expression::Eq(Box::new(lhs), Box::new(rhs)),
                Rule::gt => Expression::GT(Box::new(lhs), Box::new(rhs)),
                Rule::lt => Expression::LT(Box::new(lhs), Box::new(rhs)),
                Rule::ge => Expression::GE(Box::new(lhs), Box::new(rhs)),
                Rule::le => Expression::LE(Box::new(lhs), Box::new(rhs)),
                Rule::add => Expression::Add(Box::new(lhs), Box::new(rhs)),
                Rule::subtract => Expression::Sub(Box::new(lhs), Box::new(rhs)),
                Rule::multiply => Expression::Mul(Box::new(lhs), Box::new(rhs)),
                Rule::divide => Expression::Div(Box::new(lhs), Box::new(rhs)),
                Rule::power => Expression::Pow(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        )
    }
}

impl FromStr for Expression {
    type Err = Error<Rule>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = ExpressionParser::parse(Rule::expression, s);
        match result {
            Ok(mut res) => Ok(Self::build_expression(res.next().unwrap().into_inner())),
            Err(e) => panic!("parse failed: {}", e),
        }
    }
}

impl Default for Expression {
    fn default() -> Expression {
        Expression::Value(0.0)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Value(x) => write!(f, "{}", x),
            Expression::Var(x) => write!(f, "{}", x),
            Expression::Or(x, y) => write!(f, "{}||{}", x, y),
            Expression::And(x, y) => write!(f, "{}&&{}", x, y),
            Expression::Eq(x, y) => write!(f, "{}=={}", x, y),
            Expression::GT(x, y) => write!(f, "{}>{}", x, y),
            Expression::LT(x, y) => write!(f, "{}<{}", x, y),
            Expression::GE(x, y) => write!(f, "{}>={}", x, y),
            Expression::LE(x, y) => write!(f, "{}<={}", x, y),
            Expression::Add(x, y) => write!(f, "{}+{}", x, y),
            Expression::Sub(x, y) => write!(f, "{}-{}", x, y),
            Expression::Mul(x, y) => write!(f, "{}*{}", x, y),
            Expression::Div(x, y) => write!(f, "{}/{}", x, y),
            Expression::Pow(x, y) => write!(f, "{}^{}", x, y),
        }
    }
}

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExpressionParser {}

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(or, Left) | Operator::new(and, Left),
            Operator::new(eq, Left)
                | Operator::new(gt, Left)
                | Operator::new(lt, Left)
                | Operator::new(ge, Left)
                | Operator::new(le, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_val() {
        let expr = Expression::Value(1.0);
        let context = &vec![];
        assert_eq!(expr.eval(context), 1.0);
    }

    #[test]
    fn eval_var() {
        let expr = Expression::Var('x');
        let context = &vec![('x', 2.0)];
        assert_eq!(expr.eval(context), 2.0);
    }

    #[test]
    fn eval_add() {
        let expr = Expression::Add(
            Box::new(Expression::Var('x')),
            Box::new(Expression::Var('y')),
        );
        let context = &vec![('x', 2.0), ('y', 3.7)];
        assert_eq!(expr.eval(context), 5.7);
    }

    #[test]
    fn eval_from_str() {
        let expr: Expression = "7".parse().ok().unwrap();
        let context = &vec![];
        assert_eq!(expr.eval(context), 7.0);
    }
    #[test]
    fn eval_expression_from_str() {
        let expr: Expression = "1+2.5*3".parse().ok().unwrap();
        let context = &vec![];
        assert_eq!(expr.eval(context), 8.5);
    }
    #[test]
    fn eval_expression2_from_str() {
        let expr: Expression = "((x+1)*y^2)/5".parse().ok().unwrap();
        let context = &vec![('x', 5.0), ('y', 3.0)];
        assert_eq!(expr.eval(context), 10.8);
    }
    #[test]
    fn eval_bool_expression_from_str() {
        let expr: Expression = "1+2>5*3".parse().ok().unwrap();
        let context = &vec![];
        assert_eq!(expr.eval_bool(context), false);
    }
    #[test]
    fn eval_bool_expression_from_str2() {
        let expr: Expression = "1+2>5*3||2+2==4".parse().ok().unwrap();
        let context = &vec![];
        assert_eq!(expr.eval_bool(context), true);
    }

}
