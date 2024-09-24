#![allow(dead_code)]

use expression::{Expression, Term};
use rand::{distributions::WeightedIndex, prelude::Distribution};

#[derive(Debug)]
pub enum Error {
    WeightOutOfRange,
    ExpressionTooShort,
    OperatorMissing,
    TermMissing,
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Equation {
    expression: Expression,
    // answer: Expression,
    // answered: Option<Expression>,
}

impl Equation {
    pub fn generate(options: &ExpressionOption, weights: OperatorWeights) -> Result<Self> {
        let terms: Vec<Term> = (0..options.term_count.into())
            .map(|_| Term::random_constant(&options.constant))
            .collect();

        let mut expression = Expression::random(
            vec![Expression::Term(terms[0]), Expression::Term(terms[1])],
            &options.allowed_operators,
            &weights,
        )?;

        for term in terms.iter().skip(2) {
            expression = Expression::random(
                vec![expression, Expression::Term(*term)],
                &options.allowed_operators,
                &weights,
            )?;
        }

        Ok(Self { expression })
    }

    pub fn parse(_input: &str) -> Option<Equation> {
        // TODO: implement parse method
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operator {
    fn list_options() -> Vec<Operator> {
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

mod expression {
    use rand::Rng;

    use crate::{AllowedOperators, ConstantOption, Error, Operator, OperatorWeights, Result};

    #[derive(Debug, Clone, Copy)]
    pub struct Constant(i32);

    impl Constant {
        pub fn new(value: i32) -> Self {
            Constant(value)
        }
        pub fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    impl From<Constant> for f32 {
        fn from(value: Constant) -> Self {
            value.0 as f32
        }
    }

    impl From<Constant> for Expression {
        fn from(value: Constant) -> Self {
            Expression::Term(Term::Constant(value))
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Term {
        Constant(Constant),
    }

    impl Term {
        pub fn random_constant(constant: &ConstantOption) -> Term {
            let mut rng = rand::thread_rng();

            let constant = rng.gen_range(constant.min..=constant.max);

            Term::Constant(Constant(constant))
        }

        pub fn format_str(&self) -> String {
            match self {
                Term::Constant(constant) => constant.to_string(),
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

                fn operator(&self) -> Operator {
                    $operator
                }

                fn formatted_vec(&self) -> Vec<String> {
                    let mut response = self.0[0].formatted_vec();
                    let current_operator_order = self.operator().get_operator_order();

                    for item in self.0.iter().skip(1) {
                        response.push($symbol.to_owned());

                        let item_operator_order =
                            item.get_operator().map(|o| o.get_operator_order());

                        let item_operator_commutative =
                            item.get_operator().map(|o| o.is_commutative());

                        let has_parenthesis = item_operator_order.is_some()
                            && current_operator_order <= item_operator_order.unwrap()
                            // && current_operator_commutative
                            && item_operator_commutative == Some(false)
                            ;

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

                fn get_answer(&self) -> f32 {
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

    #[derive(Debug, Clone)]
    pub enum Expression {
        Term(Term),

        Sum(Sum),
        Subtract(Subtract),
        Multiply(Multiplication),
        Divide(Divide),
    }

    impl Expression {
        pub fn random(
            items: Vec<Expression>,
            allowed_operators: &AllowedOperators,
            options: &OperatorWeights,
        ) -> Result<Self> {
            Ok(match options.get_random_operator(allowed_operators) {
                crate::Operator::Addition => Expression::Sum(Sum::new(items)?),
                crate::Operator::Subtraction => Expression::Subtract(Subtract::new(items)?),
                crate::Operator::Multiplication => {
                    Expression::Multiply(Multiplication::new(items)?)
                }
                crate::Operator::Division => Expression::Divide(Divide::new(items)?),
            })
        }

        fn get_operator(&self) -> Option<Operator> {
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
    }
}

pub struct ConstantOption {
    min: i32,
    max: i32,
}

pub struct AllowedOperators(Vec<Operator>);
impl AllowedOperators {
    pub fn new(items: Vec<Operator>) -> Result<Self> {
        if items.len() == 0 {
            return Err(Error::OperatorMissing);
        }

        // TODO: IGNORE DUPLICATES

        Ok(AllowedOperators(items))
    }
}

#[derive(Clone, Copy)]
pub struct TermCount(u8);
impl TermCount {
    pub fn new(count: u8) -> Result<Self> {
        if count < 2 {
            return Err(Error::TermMissing);
        }

        Ok(TermCount(count))
    }
}

impl From<TermCount> for u8 {
    fn from(value: TermCount) -> Self {
        value.0
    }
}

pub struct ExpressionOption {
    constant: ConstantOption,
    allowed_operators: AllowedOperators,
    term_count: TermCount,
}

#[derive(Debug)]
pub struct OperatorWeight {
    operator: Operator,
    weight: f32,
}

impl OperatorWeight {
    pub fn new(operator: Operator, weight: f32) -> Result<Self> {
        if (-1.0..=1.0).contains(&weight) == false {
            return Err(Error::WeightOutOfRange);
        }

        Ok(Self { operator, weight })
    }
}

#[derive(Debug)]
pub struct OperatorWeights(Vec<OperatorWeight>);
impl OperatorWeights {
    pub fn new(mut weights: Vec<OperatorWeight>) -> Self {
        let missing_operators = Operator::list_options();
        let missing_operators: Vec<Operator> = missing_operators
            .iter()
            .filter(|operator| weights.iter().any(|w| w.operator == **operator) == false)
            .map(|operator| *operator)
            .collect();

        for operator in missing_operators {
            weights.push(OperatorWeight::new(operator, 1.0).unwrap());
        }

        Self(weights)
    }

    pub fn get_random_operator(&self, allowed_operators: &AllowedOperators) -> Operator {
        let options: Vec<&OperatorWeight> = self
            .0
            .iter()
            .filter(|x| allowed_operators.0.contains(&x.operator))
            .collect();

        let weights: Vec<f32> = options.iter().map(|x| x.weight).collect();

        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();

        self.0[dist.sample(&mut rng)].operator
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::{Constant, Divide, Expression, Multiplication, Subtract, Sum},
        AllowedOperators, ConstantOption, Equation, ExpressionOption, Operator, OperatorWeights,
        TermCount,
    };

    #[test]
    fn get_random_operator() {
        let weights = OperatorWeights::new(Vec::new());
        let operators = AllowedOperators::new(vec![Operator::Addition]).unwrap();
        let operator = weights.get_random_operator(&operators);

        assert!(Operator::list_options().contains(&operator));
    }

    #[test]
    fn get_random_equation() {
        for _ in 0..20 {
            let weights = OperatorWeights::new(Vec::new());
            let options = ExpressionOption {
                constant: ConstantOption { min: 1, max: 7 },
                allowed_operators: AllowedOperators(vec![
                    Operator::Addition,
                    Operator::Subtraction,
                    Operator::Multiplication,
                    Operator::Division,
                ]),
                term_count: TermCount(3),
            };

            let equation = Equation::generate(&options, weights).unwrap();

            println!(
                "{:?} = {:?}",
                equation.expression.formatted_vec().join(" "),
                equation.expression.get_answer()
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
                        Constant::new(5).into(),
                        Expression::Divide(
                            Divide::new(vec![
                                Constant::new(6).into(),
                                Expression::Subtract(
                                    Subtract::new(vec![
                                        Constant::new(7).into(),
                                        Constant::new(8).into(),
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

        assert_eq!(expression.formatted_vec().join(""), "1+2*(3-4)+5/(6/(7-8))")
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
        use crate::expression::{Constant, Divide, Expression};

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
        use crate::expression::{Constant, Expression, Multiplication};

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
    }

    mod sum {
        use crate::expression::{Constant, Expression, Sum};

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

    mod subtract {
        use crate::expression::{Constant, Expression, Subtract};

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
