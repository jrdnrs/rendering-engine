use std::{
    any::{Any, TypeId},
    collections::VecDeque,
};

const ENTITY_INDEX_BITS: u32 = 22;
const ENTITY_INDEX_MASK: u32 = (1 << ENTITY_INDEX_BITS) - 1;
const MINIMUM_FREE_SPACES: u32 = 4096;

pub struct Entity {
    pub id: u32,
}

impl Entity {
    pub fn index(&self) -> u32 {
        self.id & ENTITY_INDEX_MASK
    }

    pub fn version(&self) -> u32 {
        self.id >> ENTITY_INDEX_BITS
    }
}

pub struct EntityManager {
    entity_versions: Vec<u32>,
    free_spaces: VecDeque<u32>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entity_versions: Vec::new(),
            free_spaces: VecDeque::with_capacity(MINIMUM_FREE_SPACES as usize),
        }
    }

    #[inline]
    pub fn create(&mut self) -> Entity {
        let mut id: u32;
        if self.free_spaces.len() as u32 > MINIMUM_FREE_SPACES {
            id = self.free_spaces.pop_front().unwrap();
            id = (self.entity_versions[id as usize] << ENTITY_INDEX_BITS) | id;
        } else {
            self.entity_versions.push(0);
            id = self.entity_versions.len() as u32 - 1;
        }
        Entity { id }
    }

    #[inline]
    pub fn alive(&self, e: &Entity) -> bool {
        self.entity_versions[e.index() as usize] == e.version()
    }

    #[inline]
    pub fn destroy(&mut self, e: Entity) {
        if self.alive(&e) {
            let index = e.index();
            self.entity_versions[index as usize] += 1;
            self.free_spaces.push_back(index)
        }
    }
}
