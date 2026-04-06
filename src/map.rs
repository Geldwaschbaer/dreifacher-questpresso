use crate::event::Event;
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

    pub fn get_icon(&self, icon: &MapIcon) -> &Texture2D {
        match icon {
            MapIcon::Boss => &self.icon_boss,
            MapIcon::Endboss => &self.icon_endboss,
            MapIcon::Enemy => &self.icon_enemy,
            MapIcon::Mystery => &self.icon_mystery,
            MapIcon::Shop => &self.icon_shop,
            MapIcon::Start => &self.icon_start,
        }
    }
}

#[derive(Deserialize)]
pub struct MapBuilder(Vec<RoomBuilder>);

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

#[derive(Deserialize)]
pub enum MapIcon {
    Boss,
    Endboss,
    Enemy,
    Mystery,
    Shop,
    Start,
}

pub struct Room {
    event: Event,
    position: Vec2,
    neighbours: Vec<usize>,
    visited: bool,
    icon: MapIcon,
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

    pub fn get_icon(&self) -> &MapIcon {
        &self.icon
    }
}

#[derive(Deserialize)]
pub struct RoomBuilder {
    event_options: Vec<String>,
    position: (f32, f32),
    neighbours: Vec<usize>,
    icon: MapIcon,
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
        Room {
            event,
            position: Vec2::new(builder.position.0, builder.position.1),
            neighbours: builder.neighbours,
            visited: false,
            icon: builder.icon,
        }
    }
}
