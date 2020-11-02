extern crate bitset;


use bitset::{
    BitSet,
};

use std::collections::{
    VecDeque,
    HashMap,
};
use std::fmt;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
}

impl Entity {
    fn new(id: u32) -> Entity {
        Entity {
            id: id
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ComponentType {
    id: u16,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Signature {
    components: bitset::BitSet,
}

impl Signature {
    #[inline]
    fn new(components: BitSet) -> Signature {
        Signature {
            components: components
        }
    }

    #[inline]
    const fn empty() -> Signature {
        Signature {
            components: BitSet::new(),
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.components.count() == 0
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Signature [{}]", self.components.as_string())
    }
}

struct EntityManager {
    available_entities: VecDeque<Entity>,
    signatures: Vec<Signature>,
    living_entity_count: usize,
    living_entity_capacity: usize,
}

impl EntityManager {
    pub fn new(capacity: usize) -> EntityManager {
        let mut available_entities = VecDeque::with_capacity(capacity);
        for entity_id in 0..capacity as u32 {
            available_entities.push_back(Entity::new(entity_id));
        }

        let empty_signature = Signature::empty();
        let signatures = vec![empty_signature; capacity];

        EntityManager {
            available_entities: available_entities,
            signatures: signatures,
            living_entity_count: 0,
            living_entity_capacity: capacity,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.available_entities.pop_front();
        if id.is_none() {
            panic!("Too many entities.");
        }
        self.living_entity_count += 1;
        
        id.unwrap()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.signatures[entity.id as usize].components.reset_all();
        self.available_entities.push_back(entity);
        self.living_entity_count -= 1;
    }

    pub fn set_signature(&mut self, entity: Entity, signature: Signature) {
        self.signatures[entity.id as usize] = signature;
    }

    pub fn get_signature(&self, entity: Entity) -> Signature {
        self.signatures[entity.id as usize]
    }
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

    pub fn destroy_entity(&mut self, entity: Entity) {
        if self.entity_to_index_map.get(&entity).is_some() {
            self.remove(entity);
        }
    }
}