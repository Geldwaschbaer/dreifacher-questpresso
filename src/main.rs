#![allow(dead_code)]
#![allow(unused_variables)]

mod action;
mod map;
mod mob;
mod player;

use macroquad::prelude::*;

use crate::player::Player;

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

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(Color::from_hex(0x8d8b7f));

        player.draw();
        player.update();

        next_frame().await
    }
}
