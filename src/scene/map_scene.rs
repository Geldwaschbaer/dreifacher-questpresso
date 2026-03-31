use crate::{
    action::return_map::ReturnMap,
    dialog::{Dialog, DialogBox, DialogOption},
    map::Map,
    player::Player,
    scene::{Scene, SceneTransition, dialog_scene::DialogScene},
};
use macroquad::prelude::*;

pub struct MapScene(Map);

impl MapScene {
    pub fn new(map: Map) -> MapScene {
        MapScene(map)
    }

    pub fn get_map(&self) -> &Map {
        &self.0
    }
}

impl Scene for MapScene {
    fn draw(&self, player: &Player) {
        clear_background(Color::from_hex(0x8d8b7f));
        self.get_map().draw();
        let room = self
            .get_map()
            .get_rooms()
            .get(player.get_map_position())
            .expect("expect exists");
        for neig in room.get_neighbours() {
            let neig = self
                .get_map()
                .get_rooms()
                .get(*neig)
                .expect("element exists");
            draw_line(
                room.get_position().x,
                room.get_position().y,
                neig.get_position().x,
                neig.get_position().y,
                2.,
                Color::from_hex(0x1b252e),
            );
            draw_circle(
                neig.get_position().x,
                neig.get_position().y,
                16.,
                Color::from_hex(0x1b252e),
            );
        }
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let room = self.get_map().get_room(player.get_map_position());
            for neig_num in room.get_neighbours() {
                let target = *neig_num;
                let neig = self.get_map().get_room(target);
                let dx = neig.get_position().x - x;
                let dy = neig.get_position().y - y;
                if (dx * dx + dy * dy).sqrt() < 14.0 {
                    player.enter_room(target);
                    self.0
                        .get_rooms_mut()
                        .get_mut(target)
                        .expect("expected room to enter exists")
                        .mark_visited();
                    return SceneTransition::Push(Box::new(DialogScene::new(Dialog::new(
                        "Your Title",
                        vec![DialogBox::new(
                            "Your description here",
                            vec![DialogOption::new("Return to Map", Box::new(ReturnMap), 0)],
                        )],
                    ))));
                }
            }
        }
        SceneTransition::None
    }
}
