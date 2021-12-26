use rand::prelude::*;

#[derive(Clone, Copy)]
pub struct Ant {
    pub position: [f32; 3],
    pub colour: [f32; 3],
}

impl Ant {
    pub fn new(_goal: [f32; 3]) -> Ant {
        let mut rng = rand::thread_rng();

        Ant {
            position: [0.0, 0.0, 0.0],
            colour: [
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0)
            ],
        }
    }

    pub fn reposition(&mut self) {
        let mut rng = rand::thread_rng();

        self.position[0] += rng.gen_range(-0.01..0.01);
        self.position[1] += rng.gen_range(-0.01..0.01);
    }
}
