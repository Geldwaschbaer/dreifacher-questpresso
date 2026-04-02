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
    fn draw(&self, player: &Player) {
        clear_background(BLACK);
        draw_shadowbox_ex(
            Rect::new(
                screen_width() * 0.05,
                screen_height() * 0.5,
                screen_width() * 0.9,
                screen_height() * 0.45,
            ),
            DrawShadowboxParams {
                fill: BLACK,
                stroke: WHITE,
                ..Default::default()
            },
        );

        draw_p_ex(
            &mut Vec2::new(screen_width() * 0.5 - 55.0 * 2.0, screen_height() * 0.25),
            "Game Over",
            DrawParagraphParams {
                font_size: 55.0,
                split_line: true,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_p_ex(
            &mut Vec2::new(screen_width() * 0.15, screen_width() * 0.6),
            &format!("{}\n\nPress ESC/Q to quit.", self.reason),
            DrawParagraphParams {
                font_size: 26.0,
                split_line: true,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        SceneTransition::None
    }
}
