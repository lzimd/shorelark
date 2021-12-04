mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;
pub mod helper;

pub use self::{animal::*, food::*, world::*};
use self::{animal_individual::*, brain::*, eye::*};
use genetic_algorithm as ga;
use neural_network as nn;

use glam::{Quat, Vec3, vec3};
use rand::{Rng, RngCore};

/// How much `.step()`-s have to occur before we push data into the
/// genetic algorithm.
///
/// Value that's too low might prevent the birds from learning, while
/// a value that's too high will make the evolution unnecessarily
/// slower.
///
/// You can treat this number as "for how many steps each bird gets
/// to live"; 2500 was chosen with a fair dice roll.
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<
        ga::RouletteWheelSelection,
        ga::UniformCrossover,
        ga::GaussianMutation,
    >,
    age: usize,
}

impl Simulation {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Simulation::random(&mut rng)
    }

    pub(crate) fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng, 40, 60);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::new(),
            ga::UniformCrossover::new(),
            ga::GaussianMutation::new(0.01, 0.3),
            // ---------------------- ^--^ -^-^
            // | Chosen with a bit of experimentation.
            // |
            // | Higher values can make the simulation more chaotic,
            // | which - a bit counterintuitively - might allow for
            // | it to discover *better* solutions; but the trade-off
            // | is that higher values might also cause current, good
            // | enough solutions to be discarded.
            // ---
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn preprocess_generation(&mut self) -> bool {
        self.age += 1;
        self.age > GENERATION_LENGTH
    }

    pub fn process_generation(&mut self, animals: &[Animal]) {
        let mut rng = rand::thread_rng();
        self.age = 0;

        // Step 1: Prepare birdies to be sent into the genetic algorithm
        // Transforms `Vec<Animal>` to `Vec<AnimalIndividual>`
        let current_population: Vec<_> =
            animals.iter().map(AnimalIndividual::from_animal).collect();

        // Step 2: Evolve birdies
        // Evolves this `Vec<AnimalIndividual>`
        let (evolved_population, stats) = self.ga.evolve(&mut rng, &current_population);
        println!("statistics:{:?}", stats);
        
        // Step 3: Bring birdies back from the genetic algorithm
        // Transforms `Vec<AnimalIndividual>` back into `Vec<Animal>`
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(&mut rng))
            .collect();

        // Step 4: Restart foods
        for food in &mut self.world.foods {
            food.re_random(&mut rng);
        }
    }
}
