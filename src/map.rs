use crate::{
    draw::{ACTIVATED, AVAILABLE},
    event::Event,
};
use macroquad::prelude::*;

pub struct Map {
    rooms: Vec<Room>,
}

impl Map {
    pub async fn new() -> Map {
        //    r5
        //   / \
        //  r3 r4
        //  | \ |
        // r1  r2
        //  \  /
        //   r0
        let rooms = {
            let serialized = load_string("assets/event/welcome.json")
                .await
                .expect("file exists");
            let welcome: Event = serde_json::from_str(&serialized).expect("could not parse event");
            let serialized = load_string("assets/event/enemy.json")
                .await
                .expect("file exists");
            let combat: Event = serde_json::from_str(&serialized).expect("could not parse event");
            let mut r0 = Room::with_neighbours(Event::ReturnToMap, vec2(0.5, 0.2), vec![1, 2]);
            r0.mark_visited();
            let r1 = Room::with_neighbours(welcome, vec2(0.3, 0.4), vec![3]);
            let r2 = Room::with_neighbours(combat, vec2(0.7, 0.4), vec![3, 4]);
            let r3 = Room::with_neighbours(Event::ReturnToMap, vec2(0.3, 0.6), vec![5]);
            let r4 = Room::with_neighbours(Event::ReturnToMap, vec2(0.7, 0.6), vec![5]);
            let r5 = Room::new(Event::ReturnToMap, vec2(0.5, 0.8));
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
                    room.get_position().x * screen_width(),
                    room.get_position().y * screen_height(),
                    neig.get_position().x * screen_width(),
                    neig.get_position().y * screen_height(),
                    if choosen { 3. } else { 2. },
                    if choosen { ACTIVATED } else { AVAILABLE },
                );
            }
            draw_circle(
                room.get_position().x * screen_width(),
                room.get_position().y * screen_height(),
                14.,
                if room.is_visited() {
                    ACTIVATED
                } else {
                    AVAILABLE
                },
            );
            if room.is_visited() {
                draw_arc(
                    room.get_position().x * screen_width(),
                    room.get_position().y * screen_height(),
                    120,
                    20.,
                    20.,
                    2.,
                    320.,
                    ACTIVATED,
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

    pub fn get_room(&self, room: usize) -> &Room {
        &self.rooms.get(room).expect("room exists")
    }
}

pub struct Room {
    event: Event,
    position: Vec2,
    neighbours: Vec<usize>,
    visited: bool,
}

impl Room {
    pub fn new(event: Event, position: Vec2) -> Room {
        Room {
            event,
            position,
            neighbours: Vec::new(),
            visited: false,
        }
    }

    pub fn with_neighbours(event: Event, position: Vec2, neighbours: Vec<usize>) -> Room {
        Room {
            event,
            position,
            neighbours,
            visited: false,
        }
    }

    pub fn link_neighbour(&mut self, room: usize) {
        self.neighbours.push(room);
    }

    pub fn get_event(&self) -> &Event {
        &self.event
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_neighbours(&self) -> &Vec<usize> {
        &self.neighbours
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn mark_visited(&mut self) {
        self.visited = true;
    }
}
