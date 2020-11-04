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


use entity::{
    EntityID,
};


pub trait Component: Any + Sized {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ComponentType {
    id: u16,
}

impl ComponentType {
    fn new(id: u16) -> ComponentType {
        ComponentType {
            id: id,
        }
    }
}

impl fmt::Display for ComponentType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

pub trait ComponentStorage {
    fn destroy_entity(&mut self, entity: EntityID);
}


struct ComponentArray<T> {
    data: Vec<T>,
    entity_to_index_map: HashMap<EntityID, usize>,
    index_to_entity_map: HashMap<usize, EntityID>,
    length: usize,
    capacity: usize,
}

impl<T> ComponentArray<T> where T: Copy {
    pub fn new(capacity: usize) -> ComponentArray<T> {
        ComponentArray {
            data: Vec::with_capacity(capacity),
            entity_to_index_map: HashMap::new(),
            index_to_entity_map: HashMap::new(),
            length: 0,
            capacity: capacity,
        }
    }

    pub fn insert(&mut self, entity: EntityID, component: T) {
        let new_index = self.length;
        self.entity_to_index_map.insert(entity, new_index);
        self.index_to_entity_map.insert(new_index, entity);
        self.data[new_index] = component;
        self.length += 1;
    }

    pub fn remove(&mut self, entity: EntityID) {
        let index_of_removed_entity = self.entity_to_index_map[&entity];
        let index_of_last_element = self.length - 1;
        self.data[index_of_removed_entity] = self.data[index_of_last_element];

        let entity_of_last_element = self.index_to_entity_map[&index_of_last_element];
        self.entity_to_index_map.insert(entity_of_last_element, index_of_removed_entity);
        self.index_to_entity_map.insert(index_of_removed_entity, entity_of_last_element);

        self.entity_to_index_map.remove(&entity);
        self.index_to_entity_map.remove(&index_of_last_element);
        self.length -= 1;
    }

    pub fn get(&self, entity: EntityID) -> Option<&T> {
        self.data.get(self.entity_to_index_map[&entity])
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> ComponentStorage for ComponentArray<T> where T: Copy {
    fn destroy_entity(&mut self, entity: EntityID) {
        if self.entity_to_index_map.get(&entity).is_some() {
            self.remove(entity);
        }
    }
}
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