use crate::{
    dialog::Dialog,
    player::Player,
    scene::{SceneTransition, dialog_scene::DialogScene},
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    // nothing happens when this event is triggered
    Nothing,
    // return back to the global map
    ReturnToMap,
    // opens a new dialog
    OpenDialog(Dialog),
}

impl Event {
    pub fn trigger(&self, player: &mut Player) -> SceneTransition {
        match self {
            Event::Nothing => SceneTransition::None,
            Event::ReturnToMap => SceneTransition::Pop,
            Event::OpenDialog(dialog) => {
                SceneTransition::Push(Box::new(DialogScene::new(dialog.clone())))
            }
        }
    }
}
