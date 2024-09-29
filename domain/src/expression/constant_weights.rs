use rand::{distributions::WeightedIndex, prelude::Distribution};

use crate::{Error, Result};

use super::{options::ConstantOption, term::Constant};

#[derive(Debug)]
pub struct ConstantWeight {
    constant: Constant,
    weight: f32,
}

impl ConstantWeight {
    pub fn new(constant: Constant, weight: f32) -> Result<Self> {
        if (-1.0..=1.0).contains(&weight) == false {
            return Err(Error::WeightOutOfRange);
        }

        Ok(Self { constant, weight })
    }
}

#[derive(Debug)]
pub struct ConstantWeights(Vec<ConstantWeight>);

impl ConstantWeights {
    pub fn new(mut weights: Vec<ConstantWeight>, options: &ConstantOption) -> Self {
        // TODO: WE SHOULD VALIDATE / HANDLE CASES WHERE A OPERATOR APPEARS MORE THAT ONCE IN THE VEC

        let missing_constants = options.get_min()..=options.get_max();
        let missing_constants: Vec<Constant> = missing_constants
            .map(|operator| Constant::new(operator))
            .filter(|constant| weights.iter().any(|w| w.constant == *constant) == false)
            .collect();

        for constant in missing_constants {
            weights.push(ConstantWeight::new(constant, 1.0).unwrap());
        }

        Self(weights)
    }

    pub fn get_random(&self) -> Constant {
        if self.0.len() == 0 {
            return Constant::new(0);
        }

        let options = &self.0;

        let weights: Vec<f32> = options.iter().map(|x| x.weight).collect();

        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();

        self.0[dist.sample(&mut rng)].constant
    }
}
