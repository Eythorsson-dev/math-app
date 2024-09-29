use crate::{Error, Result};

use super::operator::Operator;

pub struct ExpressionOption {
    pub constant: ConstantOption,
    pub allowed_operators: AllowedOperators,
    pub term_count: TermCount,
}

pub struct ConstantOption {
    min: i32,
    max: i32,
}
impl ConstantOption {
    pub fn new(min: i32, max: i32) -> Result<Self> {
        if min > max {
            return Err(Error::ConstantOptionOutOfRange);
        }

        Ok(Self { min, max })
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
    pub fn get_max(&self) -> i32 {
        self.max
    }
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

    pub fn contains(&self, operator: &Operator) -> bool {
        self.0.contains(operator)
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
