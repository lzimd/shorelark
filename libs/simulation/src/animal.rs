use std::f32::consts::PI;

use crate::*;

const ANIMAL_SPEED: f32 = 0.1;

#[derive(Clone, Debug)]
pub struct Animal {
    position: Vec3,
    velocity: Vec3,
    eye: Eye,
    brain: Brain,
    /// Number of foods eaten by this animal
    pub(crate) satiation: usize,
}

impl Animal {
    fn new(rng: &mut dyn RngCore, eye: Eye, brain: Brain) -> Self {
        Self {
            position: Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 1.0),
            velocity: {
                let rotation: f32 = 2. * PI * rng.gen::<f32>();
                Vec3::new(
                    ANIMAL_SPEED * rotation.cos(),
                    ANIMAL_SPEED * rotation.sin(),
                    0.0,
                )
            },
            eye,
            brain,
            satiation: 0,
        }
    }

    pub(crate) fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self::new(rng, eye, brain)
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    pub fn eat_food(&mut self) {
        self.satiation += 1;
    }

    pub fn process_vision(&self, foods: &[Food]) -> Vec<f32> {
        self.eye.process_vision(self.position, self.velocity, foods)
    }

    pub fn brain_propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.brain.propagate(inputs)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(rng, eye, brain)
    }
}
