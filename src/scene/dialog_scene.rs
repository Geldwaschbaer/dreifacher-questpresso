use crate::{
    dialog::Dialog,
    draw::*,
    entity::player::Player,
    scene::{KEY_CODES, Scene, SceneTransition},
};
use macroquad::prelude::*;

pub struct DialogScene {
    dialog: Dialog,
    cooldown: f32,
}

impl DialogScene {
    pub fn new(dialog: Dialog) -> DialogScene {
        DialogScene {
            dialog,
            cooldown: 0.5,
        }
    }

    pub fn get_dialog(&self) -> &Dialog {
        &self.dialog
    }
}

impl Scene for DialogScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);
        draw_texture_ex(
            self.get_dialog().get_texture(),
            screen_width() * 0.05,
            screen_height() * 0.05,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width() * 0.9, screen_height() * 0.4)),
                ..Default::default()
            },
        );
        draw_shadowbox(Rect::new(
            screen_width() * 0.1,
            screen_height() * 0.5,
            screen_width() * 0.8,
            screen_height() * 0.4,
        ));
        let mut pos = Vec2::new(screen_width() * 0.15, screen_height() * 0.55);
        draw_h1(&mut pos, self.get_dialog().get_title());
        let dialog_box = self
            .get_dialog()
            .get_dialogs()
            .get(player.get_dialog_position())
            .expect("expect dialog node exists");
        draw_p(&mut pos, dialog_box.get_description());
        draw_ol(
            &mut pos,
            dialog_box.get_options().iter().map(|o| o.get_description()),
        );
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        self.cooldown = (self.cooldown - get_frame_time()).max(0.);
        let dialog_box = self
            .get_dialog()
            .get_dialogs()
            .get(player.get_dialog_position())
            .expect("expect dialog node exists");
        for (index, dialog_option) in dialog_box.get_options().iter().enumerate() {
            if is_key_down(KEY_CODES[index]) && self.cooldown == 0. {
                player.set_dialog_position(dialog_option.get_next());
                let transition = player.resolve_all(dialog_option.get_events());
                self.cooldown += 0.5;
                return transition;
            }
            if is_key_down(KeyCode::Left) {}
        }
        SceneTransition::None
    }
}
