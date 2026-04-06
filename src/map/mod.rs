pub mod icon;
pub mod node;

use crate::map::{
    icon::MapIcon,
    node::{MapNode, MapNodeBuilder},
};
use async_from::{AsyncFrom, async_trait};
use macroquad::prelude::*;
use serde::Deserialize;

pub struct Map {
    map_nodes: Vec<MapNode>,
    background: Texture2D,
    icons: [Texture2D; 7],
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

    pub fn get_map_nodes(&self) -> &Vec<MapNode> {
        &self.map_nodes
    }

    pub fn get_map_nodes_mut(&mut self) -> &mut Vec<MapNode> {
        &mut self.map_nodes
    }

    pub fn get_map_node(&self, map_node: usize) -> &MapNode {
        &self.map_nodes.get(map_node).expect("map_node exists")
    }

    pub fn get_background(&self) -> &Texture2D {
        &self.background
    }

    pub fn get_icon(&self, icon: &MapIcon) -> &Texture2D {
        &self.icons[icon.ordinal()]
    }
}

#[derive(Deserialize)]
pub struct MapBuilder(Vec<MapNodeBuilder>);

#[async_trait]
impl AsyncFrom<MapBuilder> for Map {
    async fn async_from(builder: MapBuilder) -> Map {
        let mut map_nodes = Vec::new();
        for builder in builder.0.into_iter() {
            map_nodes.push(MapNode::async_from(builder).await);
        }
        let background = load_texture("assets/backgrounds/map-bg.png")
            .await
            .expect("map background exists");
        Map {
            map_nodes,
            background,
            icons: [
                load_texture("assets/icon/boss.png")
                    .await
                    .expect("map background exists"),
                load_texture("assets/icon/endboss.png")
                    .await
                    .expect("map background exists"),
                load_texture("assets/icon/enemy.png")
                    .await
                    .expect("map background exists"),
                load_texture("assets/icon/mystery.png")
                    .await
                    .expect("map background exists"),
                load_texture("assets/icon/shop.png")
                    .await
                    .expect("map background exists"),
                load_texture("assets/icon/start.png")
                    .await
                    .expect("map background exists"),
                load_texture("assets/icon/exit.png")
                    .await
                    .expect("map background exists"),
            ],
        }
    }
}
