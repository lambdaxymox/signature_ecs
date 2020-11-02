extern crate bitset;


use bitset::{
    BitSet,
};

use std::collections::{
    VecDeque
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