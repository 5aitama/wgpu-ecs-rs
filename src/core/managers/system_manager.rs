use crate::core::{
    world::World,
};


pub trait Sys {
    fn run(&self, world: &mut World);
}

pub struct SystemManager {
    systems: Vec<Box<dyn Sys>>
}

impl SystemManager {
    
    pub fn new(max_system: usize) -> Self {
        Self {
            systems: Vec::with_capacity(max_system),
        }
    }

    pub fn register<T: Sys + 'static>(&mut self, sys: T) {
        self.systems.push(Box::new(sys));
    }

    pub fn run(&self, world: &mut World) {
        for sys in self.systems.iter() {
            sys.run(world);
        }
    }
}