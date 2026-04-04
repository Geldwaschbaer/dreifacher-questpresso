use crate::{
    draw::*,
    entity::{enemy::Enemy, player::Player},
    scene::{KEY_CODES, Scene, SceneTransition, game_over_scene::GameOverScene},
};
use macroquad::prelude::*;

pub struct CombatScene(Enemy);

impl CombatScene {
    pub fn new(mob: Enemy) -> CombatScene {
        CombatScene(mob)
    }

    pub fn get_enemy(&self) -> &Enemy {
        &self.0
    }
}

impl Scene for CombatScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);

        draw_lifebar(&mut Vec2::splat(0.), self.get_enemy().get_entity());
        draw_lifebar(
            &mut Vec2::new(screen_width() * 0.6, screen_height() * 0.4),
            player.get_entity(),
        );

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
        draw_ol(
            &mut pos,
            player
                .get_entity()
                .get_attacks()
                .iter()
                .map(|v| v.get_description()),
        );
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        if self.get_enemy().get_entity().get_health().get_cur_health() > 0 {
            // combat continues
            let attack_used = 'val: loop {
                for (index, attack) in player.get_entity().get_attacks().iter().enumerate() {
                    if is_key_pressed(KEY_CODES[index]) {
                        break 'val Some(index);
                    }
                }
                break None;
            };
            if let Some(attack) = attack_used {
                player
                    .get_entity_mut()
                    .use_attack(attack, self.0.get_entity_mut());
                let attack_count = self.get_enemy().get_entity().get_attacks().len();
                if self.get_enemy().get_entity().get_health().get_cur_health() > 0
                    && attack_count > 0
                {
                    let attack = rand::gen_range(0, attack_count);
                    self.0
                        .get_entity_mut()
                        .use_attack(attack, player.get_entity_mut());
                } else {
                    return player.resolve_all(self.get_enemy().get_on_death());
                }
                if player.get_entity().get_health().get_cur_health() <= 0 {
                    return SceneTransition::Push(Box::new(GameOverScene::new(format!(
                        "You were killed by: {}",
                        self.get_enemy().get_entity().get_name()
                    ))));
                }
            }
            SceneTransition::None
        } else {
            // killed the enemy
            SceneTransition::Pop
        }
    }
}
