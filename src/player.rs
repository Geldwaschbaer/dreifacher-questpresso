use crate::map::map::Map;
use macroquad::prelude::*;

pub struct Player {
    position: usize,
    map: Map,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: 0,
            map: Map::new(),
        }
    }

    pub fn draw(&self) {
        self.map.draw();
        let room = self
            .map
            .get_rooms()
            .get(self.position)
            .expect("expect exists");
        for neig in room.get_neighbours() {
            let neig = self.map.get_rooms().get(*neig).expect("element exists");
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

    pub fn update(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let room = self
                .map
                .get_rooms()
                .get(self.position)
                .expect("room exists");
            for neig_num in room.get_neighbours() {
                let neig = self
                    .map
                    .get_rooms()
                    .get(*neig_num)
                    .expect("neighbour exists");
                let dx = neig.get_position().x - x;
                let dy = neig.get_position().y - y;
                if (dx * dx + dy * dy).sqrt() < 14.0 {
                    self.enter_room(*neig_num);
                    return;
                }
            }
        }
    }

    fn enter_room(&mut self, room: usize) {
        self.position = room;
        self.map
            .get_rooms_mut()
            .get_mut(room)
            .expect("expected room to enter exists")
            .mark_visited();
    }

    fn leave_room(&mut self, room: usize) {}
}
