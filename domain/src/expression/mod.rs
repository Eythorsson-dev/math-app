pub mod operator;
pub mod operator_weights;
pub mod constant_weights;
pub mod options;
pub mod term;

use crate::Result;
use constant_weights::ConstantWeights;
use operator::{Divide, Multiplication, Operator, Subtract, Sum};
use operator_weights::OperatorWeights;
use options::ExpressionOption;
use term::{Constant, Term};

#[derive(Debug, Clone)]
pub enum Expression {
    Term(Term),

    Sum(Sum),
    Subtract(Subtract),
    Multiply(Multiplication),
    Divide(Divide),
}

impl Expression {
    pub fn create(operator: &Operator, items: Vec<Expression>) -> Result<Self> {
        Ok(match operator {
            Operator::Addition => Expression::Sum(Sum::new(items)?),
            Operator::Subtraction => Expression::Subtract(Subtract::new(items)?),
            Operator::Multiplication => Expression::Multiply(Multiplication::new(items)?),
            Operator::Division => Expression::Divide(Divide::new(items)?),
        })
    }

    pub fn get_operator(&self) -> Option<Operator> {
        match self {
            Expression::Term(_) => None,
            Expression::Sum(_) => Some(Operator::Addition),
            Expression::Subtract(_) => Some(Operator::Subtraction),
            Expression::Multiply(_) => Some(Operator::Multiplication),
            Expression::Divide(_) => Some(Operator::Division),
        }
    }

    pub fn formatted_vec(&self) -> Vec<String> {
        match self {
            Expression::Term(term) => vec![term.format_str()],
            Expression::Sum(sum) => sum.formatted_vec(),
            Expression::Subtract(subtract) => subtract.formatted_vec(),
            Expression::Multiply(multiplication) => multiplication.formatted_vec(),
            Expression::Divide(divide) => divide.formatted_vec(),
        }
    }

    pub fn get_answer(&self) -> f32 {
        // 1+3*(2-4)
        match self {
            Expression::Term(term) => match term {
                Term::Constant(constant) => (*constant).into(),
            },
            Expression::Sum(sum) => sum.get_answer(),
            Expression::Subtract(subtract) => subtract.get_answer(),
            Expression::Multiply(multiplication) => multiplication.get_answer(),
            Expression::Divide(divide) => divide.get_answer(),
        }
    }

    pub fn generate(options: &ExpressionOption, weights: OperatorWeights) -> Result<Self> {
        let mut operator = weights.get_random_operator(&options.allowed_operators);
        let constant_weights = ConstantWeights::new(Vec::new(), &options.constant);

        let first_constant = constant_weights.get_random();
        let mut expression: Expression = first_constant.into();

        for _ in 1..options.term_count.into() {
            let constant_weights = ConstantWeights::new(Vec::new(), &options.constant);
            let constant = if operator == Operator::Division {
                let constant: f32 = constant_weights.get_random().into();
                let divisor = expression.get_answer();

                Constant::new((constant * divisor) as i32)
            } else {
                constant_weights.get_random()
            };

            expression = Expression::create(&operator, vec![constant.into(), expression])?;
            operator = weights.get_random_operator(&options.allowed_operators);
        }

        Ok(expression)
    }

    pub fn parse(_input: &str) -> Option<Expression> {
        // TODO: implement parse method
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{
        operator::{Divide, Multiplication, Operator, Subtract, Sum},
        operator_weights::OperatorWeights,
        options::{AllowedOperators, ConstantOption, ExpressionOption, TermCount},
        term::Constant,
        Expression,
    };

    #[test]
    fn get_random_equations() {
        for _ in 0..20 {
            let weights = OperatorWeights::new(Vec::new());
            let options = ExpressionOption {
                constant: ConstantOption::new(1, 7).unwrap(),
                allowed_operators: AllowedOperators::new(vec![
                    Operator::Addition,
                    Operator::Subtraction,
                    Operator::Multiplication,
                    Operator::Division,
                ])
                .unwrap(),
                term_count: TermCount::new(3).unwrap(),
            };

            let expression = Expression::generate(&options, weights).unwrap();

            println!(
                "{:?} = {:?}",
                expression.formatted_vec().join(" "),
                expression.get_answer()
            )
        }
    }

    #[test]
    fn expression_format_vec() {
        // 1+2*(3-4)+5/(6/(7-8))
        let expression = Expression::Sum(
            Sum::new(vec![
                Constant::new(1).into(),
                Expression::Multiply(
                    Multiplication::new(vec![
                        Constant::new(2).into(),
                        Expression::Subtract(
                            Subtract::new(vec![Constant::new(3).into(), Constant::new(4).into()])
                                .unwrap(),
                        ),
                    ])
                    .unwrap(),
                ),
                Expression::Divide(
                    Divide::new(vec![
                        Constant::new(6).into(),
                        Expression::Divide(
                            Divide::new(vec![
                                Constant::new(4).into(),
                                Expression::Subtract(
                                    Subtract::new(vec![
                                        Constant::new(3).into(),
                                        Constant::new(1).into(),
                                    ])
                                    .unwrap(),
                                ),
                            ])
                            .unwrap(),
                        ),
                    ])
                    .unwrap(),
                ),
            ])
            .unwrap(),
        );

        assert_eq!(expression.formatted_vec().join(""), "1+2*(3-4)+6/(4/(3-1))")
    }

    #[test]
    fn expression_get_answer() {
        // 1+3*(2-4)+10/5/2
        let expression = Expression::Sum(
            Sum::new(vec![
                Constant::new(1).into(),
                Expression::Multiply(
                    Multiplication::new(vec![
                        Constant::new(3).into(),
                        Constant::new(-2).into(),
                        Expression::Subtract(
                            Subtract::new(vec![Constant::new(2).into(), Constant::new(4).into()])
                                .unwrap(),
                        ),
                    ])
                    .unwrap(),
                ),
                Expression::Divide(
                    Divide::new(vec![
                        Constant::new(10).into(),
                        Constant::new(5).into(),
                        Constant::new(2).into(),
                    ])
                    .unwrap(),
                ),
            ])
            .unwrap(),
        );

        println!("{:?}", expression.formatted_vec().join(""));

        assert_eq!(expression.get_answer(), 14.0)
    }

    mod division {
        use crate::expression::{operator::Divide, term::Constant, Expression};

        #[test]
        fn multiple() {
            let expression = Expression::Divide(
                Divide::new(vec![
                    Constant::new(12).into(),
                    Constant::new(3).into(),
                    Constant::new(2).into(),
                ])
                .unwrap(),
            );

            assert_eq!(2.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("12/3/2", str_value);
        }

        #[test]
        fn nested() {
            let expression = Expression::Divide(
                Divide::new(vec![
                    Constant::new(30).into(),
                    Expression::Divide(
                        Divide::new(vec![Constant::new(6).into(), Constant::new(2).into()])
                            .unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(10.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("30/(6/2)", str_value);
        }
    }

    mod multiplication {
        use crate::expression::{
            operator::{Multiplication, Sum},
            term::Constant,
            Expression,
        };

        #[test]
        fn multiple() {
            let expression = Expression::Multiply(
                Multiplication::new(vec![
                    Constant::new(12).into(),
                    Constant::new(3).into(),
                    Constant::new(2).into(),
                ])
                .unwrap(),
            );

            assert_eq!(72.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("12*3*2", str_value);
        }

        #[test]
        fn nested() {
            let expression = Expression::Multiply(
                Multiplication::new(vec![
                    Constant::new(30).into(),
                    Expression::Multiply(
                        Multiplication::new(vec![Constant::new(6).into(), Constant::new(2).into()])
                            .unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(360.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("30*6*2", str_value);
        }

        #[test]
        fn nested_sum() {
            // 4*(5+1)
            let expression = Expression::Multiply(
                Multiplication::new(vec![
                    Constant::new(4).into(),
                    Expression::Sum(
                        Sum::new(vec![Constant::new(5).into(), Constant::new(1).into()]).unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(24.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("4*(5+1)", str_value);
        }
    }

    mod sum {
        use crate::expression::{
            operator::{Subtract, Sum},
            term::Constant,
            Expression,
        };

        #[test]
        fn multiple() {
            let expression = Expression::Sum(
                Sum::new(vec![
                    Constant::new(12).into(),
                    Constant::new(3).into(),
                    Constant::new(2).into(),
                ])
                .unwrap(),
            );

            assert_eq!(17.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("12+3+2", str_value);
        }

        #[test]
        fn nested() {
            let expression = Expression::Sum(
                Sum::new(vec![
                    Constant::new(30).into(),
                    Expression::Sum(
                        Sum::new(vec![Constant::new(6).into(), Constant::new(2).into()]).unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(38.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("30+6+2", str_value);
        }

        #[test]
        fn nested_subtract() {
            let expression = Expression::Sum(
                Sum::new(vec![
                    Constant::new(30).into(),
                    Expression::Subtract(
                        Subtract::new(vec![Constant::new(6).into(), Constant::new(2).into()])
                            .unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(34.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("30+(6-2)", str_value);
        }
    }

    mod subtract {
        use crate::expression::{
            operator::{Subtract, Sum},
            term::Constant,
            Expression,
        };

        #[test]
        fn multiple() {
            let expression = Expression::Subtract(
                Subtract::new(vec![
                    Constant::new(12).into(),
                    Constant::new(3).into(),
                    Constant::new(2).into(),
                ])
                .unwrap(),
            );

            assert_eq!(7.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("12-3-2", str_value);
        }

        #[test]
        fn nested() {
            let expression = Expression::Subtract(
                Subtract::new(vec![
                    Constant::new(30).into(),
                    Expression::Subtract(
                        Subtract::new(vec![Constant::new(6).into(), Constant::new(2).into()])
                            .unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(26.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("30-(6-2)", str_value);
        }

        #[test]
        fn nested_sum() {
            let expression = Expression::Subtract(
                Subtract::new(vec![
                    Constant::new(30).into(),
                    Expression::Sum(
                        Sum::new(vec![Constant::new(6).into(), Constant::new(2).into()]).unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(22.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("30-(6+2)", str_value);
        }
    }
    mod addition {
        use crate::expression::{operator::Sum, term::Constant, Expression};

        #[test]
        fn multiple() {
            let expression = Expression::Sum(
                Sum::new(vec![
                    Constant::new(12).into(),
                    Constant::new(3).into(),
                    Constant::new(2).into(),
                ])
                .unwrap(),
            );

            assert_eq!(17.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("12+3+2", str_value);
        }

        #[test]
        fn nested() {
            let expression = Expression::Sum(
                Sum::new(vec![
                    Constant::new(30).into(),
                    Expression::Sum(
                        Sum::new(vec![Constant::new(6).into(), Constant::new(2).into()]).unwrap(),
                    ),
                ])
                .unwrap(),
            );

            assert_eq!(38.0, expression.get_answer());
            let str_value = expression.formatted_vec().join("");
            assert_eq!("30+6+2", str_value);
        }
    }

    #[test]
    fn expression_get_answer1() {
        // 5 + 6 / 6
        let expression = Expression::Sum(
            Sum::new(vec![
                Constant::new(5).into(),
                Expression::Divide(
                    Divide::new(vec![Constant::new(6).into(), Constant::new(6).into()]).unwrap(),
                ),
            ])
            .unwrap(),
        );

        assert_eq!(6.0, expression.get_answer());
        let str_value = expression.formatted_vec().join("");
        assert_eq!("5+6/6", str_value);
    }

    #[test]
    fn expression_get_answer2() {
        // 6 - 6 * 7
        let expression = Expression::Subtract(
            Subtract::new(vec![
                Constant::new(6).into(),
                Expression::Multiply(
                    Multiplication::new(vec![Constant::new(6).into(), Constant::new(7).into()])
                        .unwrap(),
                ),
            ])
            .unwrap(),
        );

        assert_eq!(-36.0, expression.get_answer());
        let str_value = expression.formatted_vec().join("");
        assert_eq!("6-6*7", str_value);
    }

    #[test]
    fn expression_get_answer_nested_multiplication() {
        let expression1 = Expression::Multiply(
            Multiplication::new(vec![
                Constant::new(10).into(),
                Constant::new(5).into(),
                Constant::new(2).into(),
            ])
            .unwrap(),
        );

        let expression2 = Expression::Multiply(
            Multiplication::new(vec![
                Constant::new(10).into(),
                Expression::Multiply(
                    Multiplication::new(vec![Constant::new(5).into(), Constant::new(2).into()])
                        .unwrap(),
                ),
            ])
            .unwrap(),
        );

        assert_eq!(expression1.get_answer(), expression2.get_answer())
    }
}
