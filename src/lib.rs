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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Signature {
    components: bitset::BitSet,
}


