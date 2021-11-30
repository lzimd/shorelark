use crate::*;

pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub(crate) fn random(rng: &mut dyn RngCore, animals_num: usize, foods_num: usize) -> Self {
        let animals = (0..animals_num).map(|_| Animal::random(rng)).collect();
        let foods = (0..foods_num).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}
