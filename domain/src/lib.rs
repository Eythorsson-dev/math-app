#![allow(dead_code)]

pub mod expression;
use expression::Expression;


#[derive(Debug)]
pub enum Error {
    WeightOutOfRange,
    ExpressionTooShort,
    OperatorMissing,
    TermMissing,
    ConstantOptionOutOfRange,
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Equation {
    pub expression: Expression,
    // answer: Expression,
    // answered: Option<Expression>,
}