use crate::{
    draw::*,
    entity::player::Player,
    scene::{Scene, SceneTransition},
};
use macroquad::prelude::*;

pub struct GameOverScene {
    reason: String,
}

impl GameOverScene {
    pub fn new(reason: String) -> GameOverScene {
        GameOverScene { reason }
    }
}

impl Scene for GameOverScene {
    fn draw(&self, _: &Player) {
        clear_background(BACKGROUND);

        draw_p_ex(
            &mut Vec2::new(screen_width() * 0.5 - 55.0 * 2.0, screen_height() * 0.25),
            "GAME OVER!",
            DrawParagraphParams {
                font_size: 55.0,
                split_line: true,
                color: RED,
                ..Default::default()
            },
        );
        draw_p_ex(
            &mut Vec2::new(screen_width() * 0.5 - 26.0 * 5.0, screen_height() * 0.45),
            &format!("{}\n\nPress ESC/Q to quit.", self.reason),
            DrawParagraphParams {
                font_size: 26.0,
                split_line: true,
                color: VIOLET,
                ..Default::default()
            },
        );
    }

    fn update(&mut self, _: &mut Player) -> SceneTransition {
        SceneTransition::None
    }
}
