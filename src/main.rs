#![allow(dead_code)]
#![allow(unused_variables)]

mod action;
mod colors;
mod dialog;
mod map;
mod mob;
mod player;
mod scene;

use macroquad::prelude::*;

use crate::{
    map::Map,
    player::Player,
    scene::{SceneBox, SceneTransition, map_scene::MapScene},
};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad Template"),
        high_dpi: true,
        sample_count: 2,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut active_scene: SceneBox = Box::new(MapScene::new(Map::new()));

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
            break;
        }

        active_scene.draw(&player);
        match active_scene.update(&mut player) {
            SceneTransition::Switch(new_scene) => active_scene = new_scene,
            SceneTransition::Return => {}
            SceneTransition::None => {}
        }

        next_frame().await
    }
}
