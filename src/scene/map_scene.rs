use crate::{
    draw::{draw_h1, draw_ol, draw_p, draw_shadowbox},
    entity::{Stat, player::Player},
    map::Map,
    scene::{Scene, SceneTransition},
};
use macroquad::prelude::*;

pub struct MapScene {
    map: Map,
    camera_pos: Vec2,
    last_pos: Option<Vec2>,
}

impl MapScene {
    pub fn new(map: Map) -> MapScene {
        MapScene {
            map,
            camera_pos: Vec2::splat(0.),
            last_pos: None,
        }
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn get_map_mut(&mut self) -> &mut Map {
        &mut self.map
    }
}

impl Scene for MapScene {
    fn draw(&self, player: &Player) {
        clear_background(WHITE);
        set_camera(&Camera2D {
            target: vec2(
                self.camera_pos.x + screen_width() / 2.0,
                self.camera_pos.y + screen_height() / 2.0,
            ),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            ..Default::default()
        });
        let room = self
            .get_map()
            .get_rooms()
            .get(player.get_map_position())
            .expect("expect exists");
        for neig in room.get_neighbours() {
            let neig = self
                .get_map()
                .get_rooms()
                .get(*neig)
                .expect("element exists");
            draw_line(
                room.get_position().x * screen_width(),
                room.get_position().y * screen_height(),
                neig.get_position().x * screen_width(),
                neig.get_position().y * screen_height(),
                3.,
                DARKPURPLE,
            );
            draw_arc(
                neig.get_position().x * screen_width(),
                neig.get_position().y * screen_height(),
                120,
                26.,
                0.,
                3.,
                360.,
                DARKPURPLE,
            )
        }
        self.get_map().draw();

        set_default_camera();
        draw_shadowbox(Rect::new(
            screen_width() * 0.8,
            screen_height() * 0.25,
            screen_width() * 0.18,
            screen_height() * 0.2,
        ));
        let mut pos = Vec2::new(screen_width() * 0.8 + 20., screen_height() * 0.25 + 40.0);
        draw_h1(&mut pos, "Legende");
        draw_ol(
            &mut pos,
            ["Endboss", "Boss", "Enemy", "Mystery"].into_iter(),
        );

        draw_shadowbox(Rect::new(
            screen_width() * 0.8,
            screen_height() * 0.55,
            screen_width() * 0.18,
            screen_height() * 0.2,
        ));
        let mut pos = Vec2::new(screen_width() * 0.8 + 20., screen_height() * 0.55 + 40.0);
        draw_h1(&mut pos, "Your Stats");
        draw_p(
            &mut pos,
            &format!(
                "Str: {}\nDex: {}\nCon: {}\nInt: {}",
                player.get_entity().get_stat(&Stat::Str),
                player.get_entity().get_stat(&Stat::Dex),
                player.get_entity().get_stat(&Stat::Con),
                player.get_entity().get_stat(&Stat::Int)
            ),
        );
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        let (x, y) = mouse_position();
        if is_mouse_button_down(MouseButton::Middle) || is_mouse_button_down(MouseButton::Right) {
            if let Some(position) = self.last_pos {
                let dx = x - position.x;
                let dy = y - position.y;
                // Just ignore horizontal movement for now.
                // self.camera_pos.x += dx;
                self.camera_pos.y += dy;
            }
        }
        self.last_pos = Some(Vec2 { x, y });
        if is_mouse_button_down(MouseButton::Left) {
            let room = self.get_map().get_room(player.get_map_position());
            for neig_num in room.get_neighbours() {
                let target = *neig_num;
                let neig = self.get_map().get_room(target);
                let dx = neig.get_position().x * screen_width() - x - self.camera_pos.x;
                let dy = neig.get_position().y * screen_height() - y - self.camera_pos.y;
                if (dx * dx + dy * dy).sqrt() < 26.0 {
                    player.set_map_position(target);
                    self.map
                        .get_rooms_mut()
                        .get_mut(target)
                        .expect("expected room to enter exists")
                        .mark_visited();
                    return self.get_map().get_room(target).get_event().trigger(player);
                }
            }
        }
        SceneTransition::None
    }
}
