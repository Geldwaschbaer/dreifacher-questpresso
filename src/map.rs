use crate::{
    draw::{ACTIVATED, AVAILABLE},
    event::Event,
};
use async_from::{AsyncFrom, async_trait};
use macroquad::prelude::*;
use serde::Deserialize;

pub struct Map {
    rooms: Vec<Room>,
    background: Texture2D,
    icon_endboss: Texture2D,
    icon_boss: Texture2D,
    icon_enemy: Texture2D,
    icon_mystery: Texture2D,
    icon_shop: Texture2D,
    icon_start: Texture2D,
}

impl Map {
    pub async fn new() -> Map {
        let builder = {
            let serialized = load_string("assets/layout/level-1.json")
                .await
                .expect("file exists");
            serde_json::from_str(&serialized).expect("could not parse event")
        };

        Map::async_from(builder).await
    }

    pub fn draw(&self) {
        for room in &self.rooms {
            let x = room.get_position().x * screen_width();
            let y = room.get_position().y * screen_height();
            for neig in room.get_neighbours() {
                let neig = self.rooms.get(*neig).expect("element exists");
                let choosen = room.is_visited() && neig.is_visited();
                draw_line(
                    x,
                    y,
                    neig.get_position().x * screen_width(),
                    neig.get_position().y * screen_height(),
                    if choosen { 3. } else { 2. },
                    if choosen { ACTIVATED } else { AVAILABLE },
                );
            }
            draw_circle(
                x,
                y,
                22.,
                if room.is_visited() {
                    ACTIVATED
                } else {
                    AVAILABLE
                },
            );
            draw_texture(&room.texture, x - 16.0, y - 16.0, WHITE);
            if room.is_visited() {
                draw_arc(x, y, 120, 26., 0., 3., 360., BLACK)
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

    pub fn get_background(&self) -> &Texture2D {
        &self.background
    }

    pub fn get_icon_boss(&self) -> &Texture2D {
        &self.icon_boss
    }

    pub fn get_icon_endboss(&self) -> &Texture2D {
        &self.icon_endboss
    }

    pub fn get_icon_enemy(&self) -> &Texture2D {
        &self.icon_enemy
    }

    pub fn get_icon_mystery(&self) -> &Texture2D {
        &self.icon_mystery
    }

    pub fn get_icon_shop(&self) -> &Texture2D {
        &self.icon_shop
    }

    pub fn get_icon_start(&self) -> &Texture2D {
        &self.icon_start
    }
}

pub struct Room {
    event: Event,
    position: Vec2,
    neighbours: Vec<usize>,
    visited: bool,
    texture: Texture2D,
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
pub struct MapBuilder(Vec<RoomBuilder>);

#[derive(Deserialize)]
pub struct RoomBuilder {
    event_options: Vec<String>,
    position: (f32, f32),
    neighbours: Vec<usize>,
    texture: String,
}

#[async_trait]
impl AsyncFrom<MapBuilder> for Map {
    async fn async_from(builder: MapBuilder) -> Map {
        let mut rooms = Vec::new();
        for builder in builder.0.into_iter() {
            rooms.push(Room::async_from(builder).await);
        }
        let background = load_texture("assets/backgrounds/map-bg.png")
            .await
            .expect("map background exists");
        background.set_filter(FilterMode::Nearest);

        let icon_boss = load_texture("assets/icon/boss.png")
            .await
            .expect("map background exists");
        icon_boss.set_filter(FilterMode::Nearest);
        let icon_endboss = load_texture("assets/icon/endboss.png")
            .await
            .expect("map background exists");
        icon_endboss.set_filter(FilterMode::Nearest);
        let icon_enemy = load_texture("assets/icon/enemy.png")
            .await
            .expect("map background exists");
        icon_enemy.set_filter(FilterMode::Nearest);
        let icon_mystery = load_texture("assets/icon/mystery.png")
            .await
            .expect("map background exists");
        let icon_shop = load_texture("assets/icon/shop.png")
            .await
            .expect("map background exists");
        icon_shop.set_filter(FilterMode::Nearest);
        let icon_start = load_texture("assets/icon/start.png")
            .await
            .expect("map background exists");
        icon_start.set_filter(FilterMode::Nearest);
        Map {
            rooms,
            background,
            icon_boss,
            icon_endboss,
            icon_enemy,
            icon_mystery,
            icon_shop,
            icon_start,
        }
    }
}

#[async_trait]
impl AsyncFrom<RoomBuilder> for Room {
    async fn async_from(builder: RoomBuilder) -> Room {
        let event = {
            let len = builder.event_options.len();
            if len > 0 {
                let element = rand::gen_range(0, builder.event_options.len());
                let file = builder
                    .event_options
                    .get(element)
                    .expect("event option exists");
                let serialized = load_string(file).await.expect("file exists");
                Event::async_from(
                    serde_json::from_str(&serialized)
                        .expect(&format!("could not parse event from file '{}'", file)),
                )
                .await
            } else {
                Event::ReturnToMap
            }
        };
        let texture = load_texture(&builder.texture)
            .await
            .expect("texture exists");
        texture.set_filter(FilterMode::Nearest);
        Room {
            event,
            position: Vec2::new(builder.position.0, builder.position.1),
            neighbours: builder.neighbours,
            visited: false,
            texture,
        }
    }
}
