use crate::entity::Entity;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Enemy {
    entity: Entity,
}

impl Enemy {
    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}
