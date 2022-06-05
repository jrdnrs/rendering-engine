use std::{
    any::{self, TypeId},
    collections::HashMap,
    iter::Zip,
    slice::IterMut,
    time,
};

use crate::{component::*, entity::*, system::*};
pub struct World {
    entity_manager: EntityManager,
    system_manager: SystemManager,
    views: HashMap<String, View>,
    current_view_name: String,
    time: time::Instant,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
            views: HashMap::new(),
            current_view_name: String::from("main"),
            time: time::Instant::now(),
        };
        w.create_view(String::from("main"));
        w.set_current_view(String::from("main")).unwrap();
        w
    }

    #[inline]
    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create()
    }

    #[inline]
    pub fn destroy_entity(&mut self, e: Entity) {
        self.entity_manager.destroy(e)
    }

    #[inline]
    pub fn is_entity_alive(&self, e: &Entity) -> bool {
        self.entity_manager.alive(&e)
    }

    /// Registers the provided component in the current view, creating a corresponding component manager
    #[inline]
    pub fn register_component<T: Component>(&mut self) {
        self.get_current_view_mut().register_component::<T>()
    }

    /// Sets the provided component for the specified entity in the current view
    #[inline]
    pub fn set_component<T: Component>(&mut self, e: &Entity, c: T) -> Result<(), String> {
        self.get_current_view_mut().set_component(e, c)
    }

    /// Removes the component of the specified type, for specified entity, in the current view
    #[inline]
    pub fn remove_component<T: Component>(&mut self, e: &Entity) -> Result<(), String> {
        self.get_current_view_mut().remove_component::<T>(e)
    }

    #[inline]
    pub fn add_system(&mut self, system: impl System) {
        self.system_manager.add_system(system)
    }

    #[inline]
    pub fn run_systems(&mut self) {
        let dt = self.time - time::Instant::now();
        self.system_manager
            .run_systems(self.views.get_mut(&self.current_view_name).unwrap(), dt)
    }

    pub fn create_view(&mut self, name: String) {
        self.views.insert(name, View::new());
    }

    pub fn set_current_view(&mut self, name: String) -> Result<(), String> {
        if self.views.contains_key(&name) {
            self.current_view_name = name;
            Ok(())
        } else {
            Err(format!("Specified view '{}' does not exist", name))
        }
    }

    pub fn get_current_view_mut(&mut self) -> &mut View {
        self.views.get_mut(&self.current_view_name).unwrap()
    }

    pub fn get_current_view_ref(&self) -> &View {
        self.views.get(&self.current_view_name).unwrap()
    }

    pub fn get_view_mut(&mut self, name: String) -> Result<&mut View, String> {
        self.views
            .get_mut(&name)
            .ok_or_else(|| format!("Specified view '{}' does not exist", name))
    }

    pub fn get_view_ref(&self, name: String) -> Result<&View, String> {
        self.views
            .get(&name)
            .ok_or_else(|| format!("Specified view '{}' does not exist", name))
    }
}

pub struct View {
    component_managers: HashMap<TypeId, ComponentManager>,
}

impl View {
    fn new() -> Self {
        View {
            component_managers: HashMap::new(),
        }
    }

    pub fn register_component<T: Component>(&mut self) {
        let comp_type = TypeId::of::<T>();
        if !self.component_managers.contains_key(&comp_type) {
            self.component_managers
                .insert(comp_type, ComponentManager::new::<T>());
        }
    }

    pub fn set_component<T: Component>(&mut self, e: &Entity, c: T) -> Result<(), String> {
        if let Some(comp_man) = self.component_managers.get_mut(&TypeId::of::<T>()) {
            comp_man.add_component(e, c);
            Ok(())
        } else {
            Err(format!(
                "The '{}' component must be registered before it can be used",
                any::type_name::<T>()
            ))
        }
    }

    pub fn remove_component<T: Component>(&mut self, e: &Entity) -> Result<(), String> {
        if let Some(comp_man) = self.component_managers.get_mut(&TypeId::of::<T>()) {
            comp_man.remove_component::<T>(e);
            Ok(())
        } else {
            Err(format!(
                "The '{}' component must be registered before it can be used",
                any::type_name::<T>()
            ))
        }
    }

    pub fn iter_components_mut<T: Component>(&mut self) -> Result<IterMut<T>, String> {
        if let Some(comp_man) = self.component_managers.get_mut(&TypeId::of::<T>()) {
            Ok(comp_man.iter_components_mut())
        } else {
            Err(format!(
                "The '{}' component must be registered before it can be used",
                any::type_name::<T>()
            ))
        }
    }

    pub fn iter_two_components_mut_zip<A, B>(
        &mut self,
    ) -> Result<Zip<IterMut<A>, IterMut<B>>, String>
    where
        A: Component,
        B: Component,
    {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();

        if !self.component_managers.contains_key(&type_a)
            || !self.component_managers.contains_key(&type_b)
        {
            return Err("Components must be registered before they can be used".to_owned());
        }

        let a = self.component_managers.get_mut(&type_a).unwrap() as *mut ComponentManager;
        let b = self.component_managers.get_mut(&type_b).unwrap() as *mut ComponentManager;

        if a == b {
            return Err("Specified components must be unique".to_owned());
        }

        // SAFETY: I think this is ok as we know that the pointers are for different vecs, and the hashmap
        // is otherwise inaccessible during use of &mut self.
        unsafe {
            let a = &mut *a;
            let b = &mut *b;

            Ok(a.iter_components_mut().zip(b.iter_components_mut()))
        }
    }

    pub fn iter_two_components_mut<A, B>(&mut self) -> Result<IterTwoCompMut<A, B>, String>
    where
        A: Component,
        B: Component,
    {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();

        if !self.component_managers.contains_key(&type_a)
            || !self.component_managers.contains_key(&type_b)
        {
            return Err("Components must be registered before they can be used".to_owned());
        }

        let a = self.component_managers.get_mut(&type_a).unwrap() as *mut ComponentManager;
        let b = self.component_managers.get_mut(&type_b).unwrap() as *mut ComponentManager;

        if a == b {
            return Err("Specified components must be unique".to_owned());
        }

        // SAFETY: I think this is ok as we know that the pointers are for different vecs, and the hashmap
        // is otherwise inaccessible during use of &mut self.
        unsafe {
            let a = &mut *a;
            let b = &mut *b;

            Ok(IterTwoCompMut::new(a, b).into_iter())
        }
    }

    pub fn iter_three_components_mut<A, B, C>(
        &mut self,
    ) -> Result<(IterMut<A>, IterMut<B>, IterMut<C>), String>
    where
        A: Component,
        B: Component,
        C: Component,
    {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();
        let type_c = TypeId::of::<C>();

        if !self.component_managers.contains_key(&type_a)
            || !self.component_managers.contains_key(&type_b)
            || !self.component_managers.contains_key(&type_c)
        {
            return Err("Components must be registered before they can be used".to_owned());
        }

        let a = self.component_managers.get_mut(&type_a).unwrap() as *mut ComponentManager;
        let b = self.component_managers.get_mut(&type_b).unwrap() as *mut ComponentManager;
        let c = self.component_managers.get_mut(&type_c).unwrap() as *mut ComponentManager;

        if a == b || a == c || b == c {
            return Err("Specified components must be unique".to_owned());
        }

        // SAFETY: I think this is ok as we know that the pointers are for different vecs, and the hashmap
        // is otherwise inaccessible during use of &mut self.
        unsafe {
            let a = &mut *a;
            let b = &mut *b;
            let c = &mut *c;

            Ok((
                a.iter_components_mut(),
                b.iter_components_mut(),
                c.iter_components_mut(),
            ))
        }
    }

    pub fn iter_four_components_mut<A, B, C, D>(
        &mut self,
    ) -> Result<(IterMut<A>, IterMut<B>, IterMut<C>, IterMut<D>), String>
    where
        A: Component,
        B: Component,
        C: Component,
        D: Component,
    {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();
        let type_c = TypeId::of::<C>();
        let type_d = TypeId::of::<D>();

        if !self.component_managers.contains_key(&type_a)
            || !self.component_managers.contains_key(&type_b)
            || !self.component_managers.contains_key(&type_c)
            || !self.component_managers.contains_key(&type_d)
        {
            return Err("Components must be registered before they can be used".to_owned());
        }

        let a = self.component_managers.get_mut(&type_a).unwrap() as *mut ComponentManager;
        let b = self.component_managers.get_mut(&type_b).unwrap() as *mut ComponentManager;
        let c = self.component_managers.get_mut(&type_c).unwrap() as *mut ComponentManager;
        let d = self.component_managers.get_mut(&type_d).unwrap() as *mut ComponentManager;

        if a == b || a == c || a == d || b == c || b == d || c == d {
            return Err("Specified components must be unique".to_owned());
        }

        // SAFETY: I think this is ok as we know that the pointers are for different vecs, and the hashmap
        // is otherwise inaccessible during use of &mut self.
        unsafe {
            let a = &mut *a;
            let b = &mut *b;
            let c = &mut *c;
            let d = &mut *d;

            Ok((
                a.iter_components_mut(),
                b.iter_components_mut(),
                c.iter_components_mut(),
                d.iter_components_mut(),
            ))
        }
    }
}
