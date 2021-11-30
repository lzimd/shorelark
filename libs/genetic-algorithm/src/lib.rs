#![feature(type_alias_impl_trait)]

mod crossover;
mod mutation;
mod select;

pub use crossover::{CrossoverMethod, UniformCrossover};
pub use mutation::{GaussianMutation, MutationMethod};
pub use select::{RouletteWheelSelection, SelectionMethod};

use rand::RngCore;
use std::ops::Index;

pub struct GeneticAlgorithm<S, C, M> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: select::SelectionMethod,
    C: crossover::CrossoverMethod,
    M: mutation::MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // selection
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                // crossover
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // mutation
                self.mutation_method.mutate(rng, &mut child);

                // convert `Chromosome` back into `Individual`
                I::create(child)
            })
            .collect()
    }
}

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{
        crossover::UniformCrossover, mutation::GaussianMutation, select::RouletteWheelSelection,
    };

    use super::*;

    impl PartialEq for Chromosome {
        fn eq(&self, other: &Self) -> bool {
            approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice(),)
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum TestIndividual {
        /// For tests that require access to chromosome
        WithChromosome { chromosome: Chromosome },

        /// For tests that don't require access to chromosome
        WithFitness { fitness: f32 },
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => chromosome,
                Self::WithFitness { .. } => {
                    panic!("not supported for TestIndividual::WithFitness")
                }
            }
        }

        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => {
                    chromosome.iter().sum()

                    // ^ the simplest fitness function ever - we're just
                    // summing all the genes together
                }

                Self::WithFitness { fitness } => *fitness,
            }
        }
    }

    fn chromosome() -> Chromosome {
        Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        }
    }

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        for _ in 1..20 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[0.59345555, 2.7889657, 4.912384]), // fitness ~= 8.3
            individual(&[1.3823613, 2.9263844, 4.951451]),  // fitness ~= 9.3
            individual(&[1.0853865, 3.069473, 4.6340246]),  // fitness ~= 8.8
            individual(&[0.59345555, 3.069473, 4.7378826]), // fitness ~= 8.4
        ];

        assert_eq!(population, expected_population);
    }

    #[test]
    fn test_no_mutation() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.0, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[2.0, 1.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        for _ in 1..7 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[2.0, 2.0, 1.0]), // fitness ~= 5
            individual(&[2.0, 2.0, 1.0]), // fitness ~= 5
            individual(&[2.0, 2.0, 1.0]), // fitness ~= 5
            individual(&[2.0, 2.0, 1.0]), // fitness ~= 5
        ];

        assert_eq!(population, expected_population);
    }

    mod len {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(chromosome().len(), 3);
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &3.0);
            assert_eq!(genes[1], &1.0);
            assert_eq!(genes[2], &2.0);
        }
    }

    mod iter_mut {
        use super::*;

        #[test]
        fn test() {
            let mut chromosome = chromosome();

            chromosome.iter_mut().for_each(|gene| {
                *gene *= 10.0;
            });

            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &30.0);
            assert_eq!(genes[1], &10.0);
            assert_eq!(genes[2], &20.0);
        }
    }

    mod index {
        use super::*;

        #[test]
        fn test() {
            let chromosome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }

    mod from_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome: Chromosome = vec![3.0, 1.0, 2.0].into_iter().collect();

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }

    mod into_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            let genes: Vec<_> = chromosome.into_iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], 3.0);
            assert_eq!(genes[1], 1.0);
            assert_eq!(genes[2], 2.0);
        }
    }
}
