use crate::*;

#[derive(Clone, Debug)]
pub struct Food {
    pub(crate) position: Vec3,
}

impl Food {
    pub fn new(food: &Food) -> Self {
        Self {
            position: food.position,
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 1.0),
        }
    }

    pub fn re_random(&mut self, rng: &mut dyn RngCore) {
        self.position = Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 1.0);
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }
}
