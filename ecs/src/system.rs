use std::time;

use crate::world::*;

pub trait System: 'static + FnMut(&mut View, time::Duration) {}
impl<T: 'static + FnMut(&mut View, time::Duration)> System for T {}
pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
}

impl SystemManager {
    pub fn new() -> Self {
        SystemManager {
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: impl System) {
        self.systems.push(Box::new(system));
    }

    pub fn run_systems(&mut self, view: &mut View, dt: time::Duration) {
        for system in self.systems.iter_mut() {
            system(view, dt)
        }
    }
}
