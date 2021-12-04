use crate::*;
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct Eye {
    pub(crate) fov_range: f32,
    pub(crate) fov_angle: f32,
    pub(crate) cells: usize,
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(&self, position: Vec3, rotation: Quat, foods: &[Food]) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];
        let x_axis = vec3(1.0, 0., 0.);

        for food in foods {
            let vec = food.position - position;
            let dist = vec.distance(Vec3::ZERO);
            if dist >= self.fov_range {
                continue;
            }

            let angle = rotation.mul_vec3(x_axis).truncate().angle_between(vec.truncate());
            let angle = helper::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / (self.fov_angle / self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(0.5, PI, 9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: Vec3::new(x, y, 1.0),
        }
    }

    const TEST_EYE_CELLS: usize = 13;

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                Vec3::new(self.x, self.y, 1.0),
                Quat::from_rotation_z(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    // As a reminder, the higher cell's value, the closer
                    // the food is:

                    if cell >= 0.7 {
                        // <0.7, 1.0>
                        // food is right in front of us
                        "#"
                    } else if cell >= 0.3 {
                        // <0.3, 0.7)
                        // food is somewhat further
                        "+"
                    } else if cell > 0.0 {
                        // <0.0, 0.3)
                        // food is pretty far away
                        "."
                    } else {
                        // 0.0
                        // no food in sight, this cell sees empty space
                        " "
                    }
                })
                .collect();

            // As before, there's nothing special about the cell values
            // (`0.7`, `0.3`, `0.0`) or the characters (`#`, `+`, `.`).
            //
            // I've chosen hash because to my eye it seems to occupy the
            // most "visual space" out of all the ASCII characters (thus
            // it represents a food being close), and then plus and dot
            // are just smaller (representing food being further away).

            // `.join()` converts `Vec<String>` into `String` using a
            // separator - e.g. `vec!["a", "b", "c"].join("|")` would
            // return `a|b|c`.
            let actual_vision = actual_vision.join("");

            // The finish line!
            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected_vision: expected_vision,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")] //
        #[test_case(1.00 * PI, "   +         ")] // Food is behind us
        #[test_case(1.25 * PI, " +           ")] // (we continue to see it
        #[test_case(1.50 * PI, "            +")] // due to 360° fov_angle.)
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")] // Here we've done 360°
        #[test_case(2.25 * PI, "        +    ")] // (and a bit more, to
        #[test_case(2.50 * PI, "      +      ")] // prove the numbers wrap.)
        fn test(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }
            .run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 0.0,
                x,
                y,
                expected_vision,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;
    
        #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_angle,
                expected_vision,
            }.run()
        }
    }
}
