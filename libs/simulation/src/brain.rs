use crate::*;

#[derive(Clone, Debug)]
pub struct Brain {
    nn: nn::Network,
    speed_accel: f32,
    rotation_accel: f32,
}

impl Brain {
    pub(crate) fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
            speed_accel: 0.2,
            rotation_accel: std::f32::consts::FRAC_PI_2,
        }
    }

    pub(crate) fn propagate(&self, inputs: Vec<f32>) -> (f32, f32) {
        let response = self.nn.propagate(inputs);

        let r0 = response[0].clamp(0.0, 1.0) - 0.5;
        let r1 = response[1].clamp(0.0, 1.0) - 0.5;
        let speed = (r0 + r1).clamp(-self.speed_accel, self.speed_accel);
        let rotation = (r0 - r1).clamp(-self.rotation_accel, self.rotation_accel);
        
        (speed, rotation)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
            speed_accel: 0.2,
            rotation_accel: 0.2,
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
