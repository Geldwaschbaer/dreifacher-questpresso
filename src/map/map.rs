use macroquad::prelude::*;
use macroquad::shapes::draw_line;

use crate::action::encounter::Encounter;
use crate::map::room::Room;

pub struct Map {
    rooms: Vec<Room>,
}

impl Map {
    pub fn new() -> Map {
        //    r5
        //   / \
        //  r3 r4
        //  | \ |
        // r1  r2
        //  \  /
        //   r0
        let rooms = {
            let mut r0 = Room::with(Box::new(Encounter::default()), vec2(200., 100.), vec![1, 2]);
            r0.mark_visited();
            let r1 = Room::with(Box::new(Encounter::default()), vec2(100., 200.), vec![3]);
            let r2 = Room::with(Box::new(Encounter::default()), vec2(300., 200.), vec![3, 4]);
            let r3 = Room::with(Box::new(Encounter::default()), vec2(100., 300.), vec![5]);
            let r4 = Room::with(Box::new(Encounter::default()), vec2(300., 300.), vec![5]);
            let r5 = Room::new(Box::new(Encounter::default()), vec2(200., 400.));
            vec![r0, r1, r2, r3, r4, r5]
        };

        Map { rooms }
    }

    pub fn draw(&self) {
        for room in &self.rooms {
            for neig in room.get_neighbours() {
                let neig = self.rooms.get(*neig).expect("element exists");
                let choosen = room.is_visited() && neig.is_visited();
                draw_line(
                    room.get_position().x,
                    room.get_position().y,
                    neig.get_position().x,
                    neig.get_position().y,
                    if choosen { 3. } else { 2. },
                    Color::from_hex(if choosen { 0x1b252e } else { 0x585858 }),
                );
            }
            draw_circle(
                room.get_position().x,
                room.get_position().y,
                14.,
                Color::from_hex(if room.is_visited() {
                    0x1b252e
                } else {
                    0x585858
                }),
            );
            if room.is_visited() {
                draw_arc(
                    room.get_position().x,
                    room.get_position().y,
                    120,
                    20.,
                    20.,
                    2.,
                    320.,
                    Color::from_hex(0x1b252e),
                )
            }
        }
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn get_rooms_mut(&mut self) -> &mut Vec<Room> {
        &mut self.rooms
    }
}
