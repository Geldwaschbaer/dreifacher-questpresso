use crate::{
    draw::*,
    entity::{Entity, enemy::Enemy, player::Player},
    scene::{Scene, SceneTransition, game_over_scene::GameOverScene},
};
use macroquad::prelude::*;

pub struct CombatScene {
    enemy: Enemy,
    cooldown: f32,
}

impl CombatScene {
    pub fn new(enemy: Enemy) -> CombatScene {
        CombatScene {
            enemy,
            cooldown: 0.0,
        }
    }

    fn draw_entity(&self, entity: &Entity, entity_pos: Vec2, mut lifebar_pos: Vec2) {
        let shadow_texture: Texture2D =
            Texture2D::from_file_with_format(include_bytes!("../../assets/icon/shadow.png"), None);
        draw_lifebar(&mut lifebar_pos, entity);
        draw_texture_ex(
            &shadow_texture,
            entity_pos.x,
            entity_pos.y + 128.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(256.0)),
                ..Default::default()
            },
        );
        // Draw texture only if the frame after cooldown is even,
        // therefore skipping odd frames and creating a flicker effect.
        if self.is_even_frame() {
            draw_texture_ex(
                entity.get_texture(),
                entity_pos.x,
                entity_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(256.0)),
                    ..Default::default()
                },
            );
        }
    }

    fn draw_battle_dialog(&self, player: &Player) {
        // Background of battle dialog.
        draw_shadowbox(Rect::new(
            screen_width() * 0.05,
            screen_height() * 0.65,
            screen_width() * 0.9,
            screen_height() * 0.3,
        ));

        let mut pos = Vec2::new(screen_width() * 0.15, screen_height() * 0.75);
        draw_h1(&mut pos, self.get_enemy().get_entity().get_name());
        draw_p(
            &mut pos,
            &format!(
                "You encountered a wild {}! What do you do?",
                self.get_enemy().get_entity().get_name()
            ),
        );
        draw_attacks(&mut pos, player);
    }

    fn update_battle(&mut self, player: &mut Player) -> SceneTransition {
        if !player.get_entity().is_alive() {
            self.update_player_killed()
        } else if self.get_enemy().get_entity().is_alive() {
            self.update_attack_used(player)
        } else {
            self.update_enemy_killed(player)
        }
    }

    fn update_player_killed(&self) -> SceneTransition {
        SceneTransition::Push(Box::new(GameOverScene::new(format!(
            "You were killed by: {}",
            self.get_enemy().get_entity().get_name()
        ))))
    }

    fn update_attack_used(&mut self, player: &mut Player) -> SceneTransition {
        // combat continues
        let attack_used = player.get_attack_used();
        if let Some(attack) = attack_used {
            self.get_enemy_mut().get_entity_mut().end_turn();
            player
                .get_entity_mut()
                .use_attack(attack, self.get_enemy_mut().get_entity_mut());
            player.get_entity_mut().end_turn();
            if self.get_enemy().get_entity().is_alive() {
                let attack_count = self.get_enemy().get_entity().get_attacks().len();
                if attack_count > 0 {
                    let attack = rand::gen_range(0, attack_count);
                    self.get_enemy_mut()
                        .get_entity_mut()
                        .use_attack(attack, player.get_entity_mut());
                }
            }
            self.cooldown = 0.5;
        }
        SceneTransition::None
    }

    fn update_enemy_killed(&self, player: &mut Player) -> SceneTransition {
        player.resolve_all(self.get_enemy().get_on_death())
    }

    fn is_even_frame(&self) -> bool {
        (&self.cooldown * 10.0) as i32 & 1 == 0
    }

    fn get_enemy(&self) -> &Enemy {
        &self.enemy
    }

    fn get_enemy_mut(&mut self) -> &mut Enemy {
        &mut self.enemy
    }
}

impl Scene for CombatScene {
    fn draw(&self, player: &Player) {
        clear_background(BACKGROUND);
        draw_texture_ex(
            player.get_combat_bg(),
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_width() / 2.0)),
                ..Default::default()
            },
        );

        self.draw_entity(
            self.get_enemy().get_entity(),
            Vec2::new(screen_width() * 0.6, screen_height() * 0.1),
            Vec2::splat(0.),
        );
        self.draw_entity(
            player.get_entity(),
            Vec2::new(screen_width() * 0.1, screen_height() * 0.25),
            Vec2::new(screen_width() * 0.6, screen_height() * 0.4),
        );
        self.draw_battle_dialog(player);
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        self.cooldown = (self.cooldown - get_frame_time()).max(0.0);
        if self.cooldown == 0.0 {
            self.update_battle(player)
        } else {
            SceneTransition::None
        }
    }
}
