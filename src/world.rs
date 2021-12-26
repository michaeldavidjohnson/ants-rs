use crate::ant::Ant;

#[derive(Clone)]
pub struct World {
    pub ants: Vec<Ant>,
}

impl World {
    pub fn new() -> World {
        World {
            ants: Vec::new(),
        }
    }
}
