// use std::{
//     any::{Any, TypeId},
//     collections::HashMap,
//     sync::Arc,
// };

// use crate::{
//     component::{Component, ComponentVec},
//     entity::Entity,
// };

// struct ArchetypeManager {
//     archetypes: HashMap<TypeId, Archetype>,
// }

// impl ArchetypeManager {
//     pub fn new() -> Self {
//         todo!()
//     }

//     /// Attaches a new component to an entity that did not have that component before
//     /// This means a change to the archetype and a subsequent move
//     pub fn add_component<T: Component>(&mut self, e: &mut Entity, c: T) {
//         let old_archetype = self.archetypes.get_mut(&e.archetype).unwrap();
//         let mut new_comp_types = Vec::new();

//         for k in old_archetype.component_lists.keys() {
//             new_comp_types.push(k.to_owned())
//         }

//         new_comp_types.push(TypeId::of::<T>());
//         new_comp_types.sort();
//         let new_archetype_id = new_comp_types.type_id();

//         old_archetype.remove_component(e);
//         if let Some(new_archetype) = self.archetypes.get_mut(&new_archetype_id) {
//             new_archetype.add_component(e, c);
//         } else {
//             let mut new_archetype = Archetype::new();
//             new_archetype.add_component(e, c);
//             self.archetypes.insert(new_archetype_id, new_archetype);
//         }
//     }

//     pub fn remove_component() {
//         todo!()
//     }
// }

// struct Archetype {
//     id: TypeId,
//     component_lists: HashMap<TypeId, Box<dyn ComponentVec>>,
//     entity_ids: Vec<u32>,
//     instance_indices: HashMap<u32, usize>,
//     edges: Vec<Edge>,
// }

// impl Archetype {
//     pub fn new() -> Self {
//         todo!()
//     }

//     pub fn add_components<T: Component>(&mut self, e: &Entity, c: Vec<(TypeId, T)>) {

//         for (type_id, component) in c.iter

//         let comp_vec: &mut Vec<T> = Self::downcast_mut(
//             self.component_lists
//                 .get_mut(&TypeId::of::<T>())
//                 .unwrap()
//                 .as_mut(),
//         );
//         comp_vec.push(c);
//         self.instance_indices.insert(e.id, comp_vec.len() - 1);
//     }

//     pub fn set_component<T: Component>(&mut self, e: &Entity, c: T) {
//         let comp_vec: &mut Vec<T> = Self::downcast_mut(
//             self.component_lists
//                 .get_mut(&TypeId::of::<T>())
//                 .unwrap()
//                 .as_mut(),
//         );
//         let comp_index = self.instance_indices.get(&e.id).unwrap();
//         comp_vec[*comp_index] = c;
//     }

//     pub fn remove_component<T: Component>(&mut self, e: &Entity) {
//         if self.includes_entity(e) {
//             let comp: &mut Vec<T> = Self::downcast_mut(
//                 self.component_lists
//                     .get_mut(&TypeId::of::<T>())
//                     .unwrap()
//                     .as_mut(),
//             );

//             let old_entity_index = self.instance_indices.remove(&e.id).unwrap();
//             comp.swap_remove(old_entity_index);
//             self.entity_ids.swap_remove(old_entity_index);

//             // update stored index for the moved entity, which is now at the old entity's index
//             let last_entity_id = self.entity_ids[old_entity_index];
//             self.instance_indices
//                 .insert(last_entity_id, old_entity_index);
//         }
//     }

//     pub fn includes_entity(&self, e: &Entity) -> bool {
//         self.instance_indices.contains_key(&e.id)
//     }

//     fn downcast_mut<T: Component>(c: &mut dyn ComponentVec) -> &mut Vec<T> {
//         c.as_any_mut().downcast_mut::<Vec<T>>().unwrap()
//     }

//     fn downcast<T: Component>(c: &dyn ComponentVec) -> &Vec<T> {
//         c.as_any_ref().downcast_ref::<Vec<T>>().unwrap()
//     }
// }

// struct Edge {}
