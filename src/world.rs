use tokio::time::Instant;

use crate::{ant::Ant, instance::Instance};

const UPDATE_INTERVAL: u128 = 100;

#[derive(Clone)]
pub struct World {
    pub ants: Vec<Ant>,
    pub timer: Instant,
}

impl World {
    pub fn new() -> World {
        let mut ants = Vec::new();

        for _ in 0..50 {
            ants.push(Ant::new([0.0, 0.0, 0.0]));
        }

        World {
            ants,
            timer: Instant::now(),
        }
    }

    pub fn update(&mut self) -> Vec<Instance> {
        let mut instances = Vec::new();

        if self.timer.elapsed().as_millis() > UPDATE_INTERVAL {
            let mut new_instances = self.ants
                .iter_mut()
                .map(|ant| {
                    ant.reposition();

                    Instance {
                        position: cgmath::Vector3 {
                            x: ant.position[0],
                            y: ant.position[1],
                            z: ant.position[2],
                        },
                        colour: ant.colour,
                    }
                })
                .collect::<Vec<_>>();

            instances.append(&mut new_instances);

            self.timer = Instant::now();
        }

        instances
    }
}
