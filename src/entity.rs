use bitset::{
    BitSet,
};


use std::collections::{
    VecDeque,
};
use std::fmt;


pub type Index = u32;

pub type Signature = BitSet;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: Index,
}

impl Entity {
    #[inline]
    fn new(id: Index) -> Entity {
        Entity {
            id: id
        }
    }

    #[inline]
    fn id(&self) -> Index {
        self.id
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.id)
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

        let empty_signature = Signature::new();
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
        self.signatures[entity.id as usize].reset_all();
        self.available_entities.push_back(entity);
        self.living_entity_count -= 1;
    }

    pub fn set_signature(&mut self, entity: Entity, signature: Signature) {
        self.signatures[entity.id as usize] = signature;
    }

    pub fn get_signature(&self, entity: Entity) -> Signature {
        self.signatures[entity.id as usize]
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        !self.available_entities.contains(&entity)
    }
}

