use domain::expression::{
    operator::Operator,
    operator_weights::OperatorWeights,
    options::{AllowedOperators, ConstantOption, ExpressionOption, TermCount},
    Expression,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExpressionDto {
    expression: Vec<String>,
    answer: f32,
}

impl From<Expression> for ExpressionDto {
    fn from(value: Expression) -> Self {
        ExpressionDto {
            expression: value.formatted_vec(),
            answer: value.get_answer(),
        }
    }
}

#[tauri::command]
pub fn get_equation() -> ExpressionDto {
    let options = ExpressionOption {
        constant: ConstantOption::new(1, 10).unwrap(),
        allowed_operators: AllowedOperators::new(vec![
            Operator::Addition,
            Operator::Subtraction,
            Operator::Multiplication,
            Operator::Division,
        ])
        .unwrap(),
        term_count: TermCount::new(2).unwrap(),
    };

    let weights = OperatorWeights::new(vec![]);

    let expression = Expression::generate(&options, weights).unwrap();

    expression.into()
}
