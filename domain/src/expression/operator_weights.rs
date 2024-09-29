use super::{operator::Operator, options::AllowedOperators};
use crate::{Error, Result};
use rand::{distributions::WeightedIndex, prelude::Distribution};

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
        // TODO: WE SHOULD VALIDATE / HANDLE CASES WHERE A OPERATOR APPEARS MORE THAT ONCE IN THE VEC

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
            .filter(|x| allowed_operators.contains(&x.operator))
            .collect();

        let weights: Vec<f32> = options.iter().map(|x| x.weight).collect();

        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();

        self.0[dist.sample(&mut rng)].operator
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{
        operator::Operator, operator_weights::OperatorWeights, options::AllowedOperators,
    };

    #[test]
    fn get_random_operator() {
        let weights = OperatorWeights::new(Vec::new());
        let operators = AllowedOperators::new(vec![Operator::Addition]).unwrap();
        let operator = weights.get_random_operator(&operators);

        assert!(Operator::list_options().contains(&operator));
    }
}
