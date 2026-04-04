use crate::{entity::Entity, event::Event};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Enemy {
    entity: Entity,
    on_death: Vec<Event>,
}

impl Enemy {
    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    pub fn get_on_death(&self) -> &Vec<Event> {
        &self.on_death
    }
}
