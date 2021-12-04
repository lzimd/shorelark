use std::f32::consts::PI;

use crate::*;

const ANIMAL_SPEED: f32 = 0.1;

#[derive(Clone, Debug)]
pub struct Animal {
    position: Vec3,
    rotation: Quat,
    speed: f32,
    eye: Eye,
    brain: Brain,
    /// Number of foods eaten by this animal
    pub(crate) satiation: usize,
}

impl Animal {
    fn new(rng: &mut dyn RngCore, eye: Eye, brain: Brain) -> Self {
        Self {
            position: Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 1.0),
            rotation: Quat::from_rotation_z(2. * PI * rng.gen::<f32>()),
            speed: ANIMAL_SPEED,
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

    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn eat_food(&mut self) {
        self.satiation += 1;
    }

    pub fn process_brains(&mut self, foods: &[Food]) {
        let vision = self.eye.process_vision(self.position, self.rotation, foods);
        let (speed, rotation) = self.brain.propagate(vision);
        self.speed = helper::wrap(self.speed + speed, 0.001, 0.2);
        self.rotation = Quat::from_rotation_z(rotation) * self.rotation;
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
