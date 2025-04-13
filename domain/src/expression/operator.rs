use crate::{expression::Expression, Error, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operator {
    pub fn list_options() -> Vec<Operator> {
        vec![
            Operator::Addition,
            Operator::Subtraction,
            Operator::Multiplication,
            Operator::Division,
        ]
    }

    pub fn get_operator_order(&self) -> u8 {
        match self {
            Operator::Multiplication => 0,
            Operator::Division => 0,
            Operator::Addition => 1,
            Operator::Subtraction => 1,
        }
    }

    /// Does the order of the operation affect the result
    pub fn is_commutative(&self) -> bool {
        match self {
            Operator::Addition => true,
            Operator::Multiplication => true,
            Operator::Division => false,
            Operator::Subtraction => false,
        }
    }
}

macro_rules! create_operator {
    ($name:ident, $symbol:expr, $operator:expr, $calculate:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name(Vec<Expression>);
        impl $name {
            pub fn new(items: Vec<Expression>) -> Result<Self> {
                if items.len() < 2 {
                    return Err(Error::ExpressionTooShort);
                }

                Ok($name(items))
            }

            pub fn operator(&self) -> Operator {
                $operator
            }

            pub fn formatted_vec(&self) -> Vec<String> {
                let mut response = self.0[0].formatted_vec();
                let current_operator = self.operator();
                let current_operator_order = current_operator.get_operator_order();
                let current_operator_commutative = current_operator.is_commutative();

                for item in self.0.iter().skip(1) {
                    response.push($symbol.to_owned());

                    let operator = item.get_operator();

                    let item_operator_order = operator.map(|o| o.get_operator_order());

                    let item_operator_commutative = operator.map(|o| o.is_commutative());

                    let has_parenthesis = item_operator_order.is_some()
                        && current_operator_order <= item_operator_order.unwrap()
                        && (current_operator_commutative == false
                            || item_operator_commutative == Some(false)
                            || (current_operator_order < item_operator_order.unwrap()));

                    if has_parenthesis {
                        response.push("(".to_owned());
                        response.append(&mut item.formatted_vec());
                        response.push(")".to_owned());
                    } else {
                        response.append(&mut item.formatted_vec());
                    }
                }

                response
            }

            pub fn get_answer(&self) -> f32 {
                let mut answer: f32 = self.0[0].get_answer();

                for item in self.0.iter().skip(1) {
                    answer = $calculate(answer, item.get_answer());
                }

                answer
            }
        }
    };
}

create_operator!(Sum, "+", Operator::Addition, |a, b| a + b);
create_operator!(Subtract, "-", Operator::Subtraction, |a, b| a - b);
create_operator!(Multiplication, "*", Operator::Multiplication, |a, b| a * b);
create_operator!(Divide, "/", Operator::Division, |a, b| a / b);

mod tests {
    use crate::expression::{
        operator::Subtract,
        term::{Constant, Term},
        Expression,
    };

    #[test]
    pub fn test() {
        let value = Subtract::new(vec![
            Expression::Term(Term::Constant(Constant::new(111))),
            Expression::Term(Term::Constant(Constant::new(222))),
            Expression::Term(Term::Constant(Constant::new(333))),
        ])
        .unwrap();

        println!("{:?}", value.formatted_vec());
        println!("{:?}", value.get_answer())
    }
}
