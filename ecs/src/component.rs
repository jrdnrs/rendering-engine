use std::{any::Any, collections::HashMap, slice::IterMut};

use crate::entity::*;

pub trait Component: 'static {}
impl<T: Any> Component for T {}

pub trait ComponentVec {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T: Component> ComponentVec for Vec<T> {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ComponentManager {
    components: Box<dyn ComponentVec>,
    pub entity_ids: Vec<u32>,
    instance_indices: HashMap<u32, usize>,
}

impl ComponentManager {
    pub fn new<T: Component>() -> Self {
        ComponentManager {
            components: Box::new(Vec::<T>::new()),
            entity_ids: Vec::new(),
            instance_indices: HashMap::new(),
        }
    }

    /// Adds the entity to this component manager and the corresponding component instance
    pub fn add_component<T: Component>(&mut self, e: &Entity, c: T) {
        let comp_vec: &mut Vec<T> = Self::downcast_mut(self.components.as_mut());

        comp_vec.push(c);
        self.entity_ids.push(e.id);
        self.instance_indices.insert(e.id, comp_vec.len() - 1);
    }

    /// This assumes that the component already exists for the entity, and will panic otherwise. <br>
    /// TODO: Should we check instead?
    pub fn set_component<T: Component>(&mut self, e: &Entity, c: T) {
        let comp_vec: &mut Vec<T> = Self::downcast_mut(self.components.as_mut());
        let comp_index = self.instance_indices.get(&e.id).unwrap();
        comp_vec[*comp_index] = c;
    }

    /// This does check if the component already exists for the entity, but will do nothing if not
    pub fn remove_component<T: Component>(&mut self, e: &Entity) {
        if self.includes_entity(e) {
            let comp: &mut Vec<T> = Self::downcast_mut(self.components.as_mut());

            let old_entity_index = self.instance_indices.remove(&e.id).unwrap();
            comp.swap_remove(old_entity_index);
            self.entity_ids.swap_remove(old_entity_index);

            // update stored index for the moved entity, which is now at the old entity's index
            let last_entity_id = self.entity_ids[old_entity_index];
            self.instance_indices
                .insert(last_entity_id, old_entity_index);
        }
    }

    pub fn includes_entity(&self, e: &Entity) -> bool {
        self.instance_indices.contains_key(&e.id)
    }

    /// Returns mutable iterator for ComponentVec
    pub fn iter_components_mut<T: Component>(&mut self) -> IterMut<T> {
        Self::downcast_mut(self.components.as_mut()).iter_mut()
    }

    /// Returns mutable reference to ComponentVec
    pub fn get_components_mut<T: Component>(&mut self) -> &mut Vec<T> {
        Self::downcast_mut(self.components.as_mut())
    }

    /// Returns mutable reference to specific instance of component belonging to the entity
    pub fn get_component_mut<T: Component>(&mut self, e: &Entity) -> Result<&mut T, String> {
        if let Some(index) = self.instance_indices.get(&e.id) {
            let store: &mut Vec<T> = Self::downcast_mut(self.components.as_mut());
            Ok(store.get_mut(*index).unwrap())
        } else {
            Err(format!(
                "Specified entity 'index: {}, gen: {}' does not exist",
                e.index(),
                e.version()
            ))
        }
    }

    /// Returns immutable reference to specific instance of component belonging to the entity
    pub fn get_component_ref<T: Component>(&self, e: &Entity) -> Result<&T, String> {
        if let Some(index) = self.instance_indices.get(&e.id) {
            let store: &Vec<T> = Self::downcast(self.components.as_ref());
            Ok(store.get(*index).unwrap())
        } else {
            Err(format!(
                "Specified entity 'index: {}, gen: {}' does not exist",
                e.index(),
                e.version()
            ))
        }
    }

    /// Runtime reflection of trait object's (ComponentVec) true type  and downcasts to concrete type of Vec<Component>
    fn downcast_mut<T: Component>(c: &mut dyn ComponentVec) -> &mut Vec<T> {
        c.as_any_mut().downcast_mut::<Vec<T>>().unwrap()
    }

    /// Runtime reflection of trait object's (ComponentVec) true type  and downcasts to concrete type of Vec<Component>
    fn downcast<T: Component>(c: &dyn ComponentVec) -> &Vec<T> {
        c.as_any_ref().downcast_ref::<Vec<T>>().unwrap()
    }
}

/// This is really slow :/
pub struct IterTwoCompMut<'a, A: Component, B: Component> {
    shortest: &'a mut Vec<A>,
    shortest_entity_ids: &'a Vec<u32>,
    other: &'a mut Vec<B>,
    other_instance_indices: &'a HashMap<u32, usize>,
    index: usize,
}

impl<'a, A: Component, B: Component> IterTwoCompMut<'a, A, B> {
    pub fn new(shortest: &'a mut ComponentManager, other: &'a mut ComponentManager) -> Self {
        IterTwoCompMut {
            shortest: unsafe { &mut *(shortest.get_components_mut::<A>() as *mut _) },
            shortest_entity_ids: unsafe { &*(&mut shortest.entity_ids as *const _) },
            other: unsafe { &mut *(other.get_components_mut::<B>() as *mut _) },
            other_instance_indices: unsafe { &*(&mut other.instance_indices as *const _) },
            index: 0,
        }
    }
}

impl<'a, A: Component, B: Component> Iterator for IterTwoCompMut<'a, A, B> {
    type Item = (&'a mut A, &'a mut B);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.shortest.len() {
            unsafe {
                let a_entity = self.shortest_entity_ids.get_unchecked(self.index);
                let b_comp = if let Some(other_index) = self.other_instance_indices.get(a_entity) {
                    &mut *(self.other.get_unchecked_mut(*other_index) as *mut B)
                } else {
                    self.index += 1;
                    continue;
                };
                let a_comp = &mut *(self.shortest.get_unchecked_mut(self.index) as *mut A);

                self.index += 1;
                return Some((a_comp, b_comp));
            }
        }

        None
    }
}
