pub mod components;
pub mod entity_generator;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Entity(u32);
