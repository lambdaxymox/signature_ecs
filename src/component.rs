use std::any::Any;
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

