use crate::{
    draw::*,
    entity::{Stat, player::Player},
    map::{Map, Room},
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

    fn draw_map(&self, player: &Player) {
        for room in self.get_map().get_rooms() {
            self.draw_connections(room, player);
        }
        for room in self.get_map().get_rooms() {
            self.draw_room(room, player);
        }
    }

    fn draw_connections(&self, room: &Room, player: &Player) {
        for neig in room.get_neighbours() {
            let neig = self
                .get_map()
                .get_rooms()
                .get(*neig)
                .expect("element exists");
            let is_choosen = room.is_visited() && neig.is_visited();
            let is_option = self.is_player_option(neig, player);
            self.draw_path(
                room,
                neig,
                is_option || is_choosen,
                if is_option && room.is_visited() {
                    VIOLET
                } else if is_choosen {
                    ACTIVATED
                } else {
                    AVAILABLE
                },
            );
        }
    }

    fn draw_room(&self, room: &Room, player: &Player) {
        let x = room.get_position().x * screen_width();
        let y = room.get_position().y * screen_height();
        let is_option = self.is_player_option(room, player);
        if is_option {
            draw_arc(x, y, 120, 26., 0., 3., 360., VIOLET)
        }
        draw_circle(
            x,
            y,
            22.,
            if is_option {
                VIOLET
            } else if room.is_visited() {
                ACTIVATED
            } else {
                AVAILABLE
            },
        );
        draw_texture(room.get_icon(), x - 16.0, y - 16.0, WHITE);
        if room.is_visited() {
            draw_arc(x, y, 120, 26., 0., 3., 360., BLACK)
        }
    }

    fn draw_path(&self, start: &Room, end: &Room, thicker: bool, color: Color) {
        draw_line(
            start.get_position().x * screen_width(),
            start.get_position().y * screen_height(),
            end.get_position().x * screen_width(),
            end.get_position().y * screen_height(),
            if thicker { 4. } else { 2. },
            color,
        );
    }

    fn is_player_option(&self, room: &Room, player: &Player) -> bool {
        let player_position = self
            .get_map()
            .get_rooms()
            .get(player.get_map_position())
            .expect("expect exists");
        for neig in player_position.get_neighbours() {
            let neig = self.get_map().get_room(*neig);
            if std::ptr::eq(neig, room) {
                return true;
            }
        }
        false
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
        clear_background(BACKGROUND);
        draw_texture_ex(
            self.map.get_background(),
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        set_camera(&Camera2D {
            target: vec2(
                self.camera_pos.x + screen_width() / 2.0,
                self.camera_pos.y + screen_height() / 2.0,
            ),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            ..Default::default()
        });
        self.draw_map(player);

        set_default_camera();
        draw_shadowbox(Rect::new(
            screen_width() * 0.8,
            screen_height() * 0.15,
            screen_width() * 0.18,
            screen_height() * 0.3,
        ));
        let mut pos = Vec2::new(screen_width() * 0.8 + 10., screen_height() * 0.15 + 40.0);
        draw_h1(&mut pos, "  Legend");
        for (index, icon) in [
            self.get_map().get_icon_boss(),
            self.get_map().get_icon_enemy(),
            self.get_map().get_icon_mystery(),
            self.get_map().get_icon_shop(),
            self.get_map().get_icon_start(),
        ]
        .into_iter()
        .enumerate()
        {
            draw_texture(icon, pos.x, pos.y - 24.0 + 32.0 * index as f32, HEADER_COL);
        }
        pos.x += 34.0;
        draw_p_ex(
            &mut pos,
            "Boss\nEnemy\nMystery\nShop\nStart",
            DrawParagraphParams {
                font_size: 32.0,
                ..Default::default()
            },
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
                // Just ignore horizontal movement for now.
                let dy = y - position.y;
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
