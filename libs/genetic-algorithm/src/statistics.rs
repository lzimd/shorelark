use crate::*;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
}

impl Statistics {
    pub(crate) fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let len = population.len();

        let fitnesses = {
            let mut fitnesses: Vec<_> = population.iter().map(|f| f.fitness()).collect();
            fitnesses.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            fitnesses
        };

        let min_fitness = fitnesses[0];
        let max_fitness = fitnesses[len - 1];
        let avg_fitness = fitnesses.iter().sum::<f32>() / (len as f32);

        Self {
            min_fitness,
            max_fitness,
            avg_fitness,
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }
}
