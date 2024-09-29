use rand::Rng;

use super::{options::ConstantOption, Expression};

#[derive(Debug, Clone, Copy)]
pub enum Term {
    Constant(Constant),
}

impl Term {
    pub fn random_constant(constant: &ConstantOption) -> Term {
        let mut rng = rand::thread_rng();

        let constant = rng.gen_range(constant.get_min()..=constant.get_max());

        Term::Constant(Constant(constant))
    }

    pub fn format_str(&self) -> String {
        match self {
            Term::Constant(constant) => constant.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Constant(i32);

impl Constant {
    pub fn new(value: i32) -> Self {
        Constant(value)
    }
    pub fn to_string(self) -> String {
        self.0.to_string()
    }
}

impl From<Constant> for i32 {
    fn from(value: Constant) -> Self {
        value.0
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
