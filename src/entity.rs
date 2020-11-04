use bitset::{
    BitSet,
};


use std::collections::{
    VecDeque,
    HashMap,
};
use std::fmt;


pub type Index = u32;

pub type Signature = BitSet;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityID {
    id: Index,
}

impl EntityID {
    #[inline]
    fn new(id: Index) -> EntityID {
        EntityID {
            id: id
        }
    }

    #[inline]
    fn id(&self) -> Index {
        self.id
    }
}

impl fmt::Display for EntityID {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

struct EntityManager {
    available_entities: VecDeque<EntityID>,
    signatures: Vec<Signature>,
    living_entity_count: usize,
    living_entity_capacity: usize,
}

impl EntityManager {
    pub fn new(capacity: usize) -> EntityManager {
        let mut available_entities = VecDeque::with_capacity(capacity);
        for entity_id in 0..capacity as u32 {
            available_entities.push_back(EntityID::new(entity_id));
        }

        let empty_signature = Signature::new();
        let signatures = vec![empty_signature; capacity];

        EntityManager {
            available_entities: available_entities,
            signatures: signatures,
            living_entity_count: 0,
            living_entity_capacity: capacity,
        }
    }

    pub fn create_entity(&mut self) -> EntityID {
        let id = self.available_entities.pop_front();
        if id.is_none() {
            panic!("Too many entities.");
        }
        self.living_entity_count += 1;
        
        id.unwrap()
    }

    pub fn destroy_entity(&mut self, entity: EntityID) {
        self.signatures[entity.id as usize].reset_all();
        self.available_entities.push_back(entity);
        self.living_entity_count -= 1;
    }

    pub fn set_signature(&mut self, entity: EntityID, signature: Signature) {
        self.signatures[entity.id as usize] = signature;
    }

    pub fn get_signature(&self, entity: EntityID) -> Signature {
        self.signatures[entity.id as usize]
    }
}

