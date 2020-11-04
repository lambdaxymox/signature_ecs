extern crate bitset;


use bitset::{
    BitSet,
};

use std::any;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{
    VecDeque,
    HashMap,
};
use std::fmt;

mod entity;
mod component;
mod storage;


use entity::{
    Entity,
};


/*
struct ComponentManager {
    component_types: HashMap<&'static str, ComponentType>,
    component_arrays: HashMap<&'static str, Rc<RefCell<dyn ComponentStorage>>>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    fn get_component_array<T>(&self) -> Rc<RefCell<dyn ComponentStorage>> {
        let type_name = any::type_name::<T>();
        self.component_arrays[&type_name].clone()
    }

    pub fn register_component<T: 'static + Copy>(&mut self, capacity: usize) {
        let type_name = any::type_name::<T>();
        assert!(self.component_types.contains_key(type_name));
        self.component_types.insert(type_name, self.next_component_type);
        let array = ComponentArray::<T>::new(capacity);
        self.component_arrays.insert(type_name, Rc::new(RefCell::new(array)));
        self.next_component_type = ComponentType::new(self.next_component_type.id + 1);
    }

    pub fn get_component_type<T>(&self) -> ComponentType {
        let type_name = any::type_name::<T>();
        self.component_types[type_name]
    }

    pub fn add_component<T>(&self, entity: EntityID, component: T) {
        let mut array = self.get_component_array::<T>().borrow_mut();
        array.insert(entity, component);
    }

    pub fn remove_component<T: Copy>(&mut self, entity: EntityID) {
        let mut array = self.get_component_array::<T>().borrow_mut();
        array.remove(entity);
    }

    pub fn get_component<T>(&self, entity: EntityID) -> &T {
        self.get_component_array::<T>().get(entity)
    }

    pub fn destroy_entity(&mut self, entity: EntityID) {
        for cell in self.component_arrays.values() {
            let mut component = cell.borrow_mut();
            component.destroy_entity(entity);
        }
    }
}
*/