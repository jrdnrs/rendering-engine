use crate::components::Renderable;

pub struct DrawCommands {
    pub indices: Vec<usize>,
    pub renderable_keys: Vec<u32>,
    hash_fn: fn(&Renderable) -> u32,
}

impl DrawCommands {
    pub fn new(hash_fn: fn(&Renderable) -> u32) -> Self {
        Self {
            indices: Vec::new(),
            renderable_keys: Vec::new(),
            hash_fn,
        }
    }

    pub fn sort_indices(&mut self) {
        if self.renderable_keys.len() > self.indices.len() {
            for i in self.indices.len()..self.renderable_keys.len() {
                self.indices.push(i)
            }
        } else if self.renderable_keys.len() < self.indices.len() {
            self.indices.clear();
            for i in 0..self.renderable_keys.len() {
                self.indices.push(i)
            }
        }

        self.indices.sort_by_key(|k| self.renderable_keys[*k])
    }

    pub fn update_keys(&mut self, renderables: &[Renderable]) {
        self.renderable_keys.clear();
        for renderable in renderables {
            self.renderable_keys.push((self.hash_fn)(renderable))
        }
    }
}
