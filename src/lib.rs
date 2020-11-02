extern crate bitset;


use std::collections::{
    VecDeque
};
use std::fmt;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
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

impl fmt::Display for Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Signature [{}]", self.components.as_string())
    }
}

struct EntityManager {
    available_entities: VecDeque<Entity>,
    signatures: Vec<Signature>,
    living_entity_count: u32,
}

