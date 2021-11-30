use crate::*;

#[derive(Clone, Debug)]
pub struct Brain {
    nn: nn::Network,
}

impl Brain {
    pub(crate) fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    pub(crate) fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.nn.propagate(inputs)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
