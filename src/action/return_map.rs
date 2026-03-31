use crate::{action::Action, player::Player, scene::SceneTransition};

pub struct ReturnMap;

impl Action for ReturnMap {
    fn activate(&self, player: &mut Player) -> SceneTransition {
        SceneTransition::Pop
    }
}
