use crate::entity::{
    Entity,
};

use std::any::Any;
use std::collections::{
    HashMap,
};
use std::fmt;


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
    fn destroy_entity(&mut self, entity: Entity);
}


struct ComponentArray<T> {
    data: Vec<T>,
    entity_to_index_map: HashMap<Entity, usize>,
    index_to_entity_map: HashMap<usize, Entity>,
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

    pub fn insert(&mut self, entity: Entity, component: T) {
        let new_index = self.length;
        self.entity_to_index_map.insert(entity, new_index);
        self.index_to_entity_map.insert(new_index, entity);
        self.data[new_index] = component;
        self.length += 1;
    }

    pub fn remove(&mut self, entity: Entity) {
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

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(self.entity_to_index_map[&entity])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(self.entity_to_index_map[&entity])
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> ComponentStorage for ComponentArray<T> where T: Copy {
    fn destroy_entity(&mut self, entity: Entity) {
        if self.entity_to_index_map.get(&entity).is_some() {
            self.remove(entity);
        }
    }
}

