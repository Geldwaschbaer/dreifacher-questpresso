pub mod combat_scene;
pub mod dialog_scene;
pub mod game_over_scene;
pub mod map_scene;

use crate::{entity::player::Player, scene::map_scene::MapScene};
use macroquad::prelude::*;

pub const KEY_CODES: [KeyCode; 9] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
];

pub trait Scene {
    fn draw(&self, player: &Player);
    fn update(&mut self, player: &mut Player) -> SceneTransition;
}

pub enum SceneTransition {
    // No transition is happening
    None,
    Push(SceneBox),
    Pop,
    Replace(SceneBox),
}

pub type SceneBox = Box<dyn Scene>;

pub struct SceneManager {
    map: MapScene,
    stack: Vec<SceneBox>,
}

impl SceneManager {
    pub fn new(scene: MapScene) -> SceneManager {
        SceneManager {
            map: scene,
            stack: Vec::new(),
        }
    }

    pub fn trigger_first_room(&mut self, player: &mut Player) {
        let first_room = self
            .map
            .get_map_mut()
            .get_rooms_mut()
            .first_mut()
            .expect("expect any room exists");
        first_room.mark_visited();
        let transition = first_room.get_event().trigger(player);
        self.manage_transition(transition);
    }

    fn manage_transition(&mut self, transition: SceneTransition) {
        match transition {
            SceneTransition::Push(scene) => self.stack.push(scene),
            SceneTransition::Pop => {
                self.stack.pop();
            }
            SceneTransition::Replace(scene) => {
                self.stack.pop();
                self.stack.push(scene);
            }
            SceneTransition::None => {}
        };
    }
}

impl Scene for SceneManager {
    fn draw(&self, player: &Player) {
        if self.stack.is_empty() {
            self.map.draw(player);
        } else {
            self.stack.last().expect("vec not empty").draw(player);
        }
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        let transition = if self.stack.is_empty() {
            self.map.update(player)
        } else {
            self.stack.last_mut().expect("vec not empty").update(player)
        };
        self.manage_transition(transition);

        // return value is ignored
        SceneTransition::None
    }
}
