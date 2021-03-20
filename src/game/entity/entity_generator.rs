use std::collections::BTreeSet;
use super::Entity;


fn maximum<T>(set: &BTreeSet<T>) -> Option<&T> {
    set.iter().next_back()
}


#[derive(Default)]
pub struct EntityGenerator {
    used_ids: BTreeSet<u32>,
}

impl EntityGenerator {
    pub fn new() -> Self { Self::default() }

    pub fn generate(&mut self) -> Entity {
        let max_id = maximum(&self.used_ids).cloned().unwrap_or(0);
        self.used_ids.insert(max_id + 1);
        Entity(max_id + 1)
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.used_ids.remove(&entity.0);
    }
}