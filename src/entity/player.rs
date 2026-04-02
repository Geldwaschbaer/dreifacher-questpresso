use crate::entity::{Attack, Entity, Health};

pub struct Player {
    map_position: usize,
    dialog_position: usize,
    entity: Entity,
}

impl Player {
    pub fn new() -> Player {
        Player {
            map_position: 0,
            dialog_position: 0,
            entity: Entity::new(
                "Player".into(),
                Health::new(20),
                vec![
                    Attack::new("Punch them with your fist!".into(), 10, 0, false),
                    Attack::new("Trink a heal potion!".into(), 0, 5, false),
                    Attack::new("Drain the life of your enemies!".into(), 3, 3, true),
                ],
            ),
        }
    }

    pub fn enter_room(&mut self, room: usize) {
        self.map_position = room;
    }

    pub fn get_map_position(&self) -> usize {
        self.map_position
    }

    pub fn get_dialog_position(&self) -> usize {
        self.dialog_position
    }

    pub fn set_dialog_position(&mut self, position: usize) {
        self.dialog_position = position;
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}
