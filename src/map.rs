use crate::{
    draw::{ACTIVATED, AVAILABLE},
    event::Event,
};
use async_from::{AsyncFrom, async_trait};
use macroquad::prelude::*;
use serde::Deserialize;

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
        let layout = {
            let serialized = load_string("assets/layout/level-1.json")
                .await
                .expect("file exists");
            serde_json::from_str(&serialized).expect("could not parse event")
        };

        Map::async_from(layout).await
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

#[derive(Deserialize)]
pub struct Layout(Vec<RoomLayout>);

#[derive(Deserialize)]
pub struct RoomLayout {
    event_options: Vec<String>,
    position: (f32, f32),
    neighbours: Vec<usize>,
}

#[async_trait]
impl AsyncFrom<Layout> for Map {
    async fn async_from(layout: Layout) -> Map {
        let mut rooms = Vec::new();
        for layout in layout.0.into_iter() {
            rooms.push(Room::async_from(layout).await);
        }
        Map { rooms }
    }
}

#[async_trait]
impl AsyncFrom<RoomLayout> for Room {
    async fn async_from(layout: RoomLayout) -> Room {
        let event = {
            let len = layout.event_options.len();
            if len > 0 {
                let element = rand::gen_range(0, layout.event_options.len());
                let file = layout
                    .event_options
                    .get(element)
                    .expect("event option exists");
                let serialized = load_string(file).await.expect("file exists");
                serde_json::from_str(&serialized)
                    .expect(&format!("could not parse event from file '{}'", file))
            } else {
                Event::ReturnToMap
            }
        };
        Room {
            event,
            position: Vec2::new(layout.position.0, layout.position.1),
            neighbours: layout.neighbours,
            visited: false,
        }
    }
}
