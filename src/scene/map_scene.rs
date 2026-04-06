use crate::{
    draw::*,
    entity::{player::Player, stat::Stat},
    map::{Map, icon::MapIcon, node::MapNode},
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

    fn draw_background(&self) {
        clear_background(BACKGROUND);
        draw_texture_ex(
            self.map.get_background(),
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_width() * 2.)),
                ..Default::default()
            },
        );
    }

    fn draw_map(&self, player: &Player) {
        // Set camera for map movement.
        set_camera(&Camera2D {
            target: vec2(
                self.camera_pos.x + screen_width() / 2.0,
                self.camera_pos.y + screen_height() / 2.0,
            ),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            ..Default::default()
        });
        // We first draw the connections and then the map_nodes themself,
        // because otherwise the connections would be drawn over the map_node icons.
        for map_node in self.get_map().get_map_nodes() {
            self.draw_connections(map_node, player);
        }
        for map_node in self.get_map().get_map_nodes() {
            self.draw_map_node(map_node, player);
        }
        // Reset camera.
        set_default_camera();
    }

    fn draw_connections(&self, map_node: &MapNode, player: &Player) {
        for neig in map_node.get_neighbours() {
            let neig = self.get_map().get_map_node(*neig);
            let is_choosen = map_node.is_visited() && neig.is_visited();
            let is_option = self.is_player_option(neig, player)
                && std::ptr::eq(
                    self.get_map().get_map_node(player.get_map_position()),
                    map_node,
                );
            self.draw_path(
                map_node,
                neig,
                is_option || is_choosen,
                if is_option {
                    NEXT_COL
                } else if is_choosen {
                    ACTIVATED
                } else {
                    AVAILABLE
                },
            );
        }
    }

    fn draw_map_node(&self, map_node: &MapNode, player: &Player) {
        let x = map_node.get_position().x * screen_width();
        let y = map_node.get_position().y * screen_height();
        let is_option = self.is_player_option(map_node, player);
        draw_circle(
            x,
            y,
            22.,
            if is_option {
                NEXT_COL
            } else if map_node.is_visited() {
                ACTIVATED
            } else {
                AVAILABLE
            },
        );
        // If this map_node can be choosen by the player or was already visited, draw a ring around it.
        if is_option || map_node.is_visited() {
            draw_arc(
                x,
                y,
                120,
                26.,
                0.,
                3.,
                360.,
                if is_option {
                    NEXT_COL
                } else {
                    Color::from_rgba(0, 0, 0, 0)
                },
            )
        }
        // Draw the map icon.
        draw_texture(
            self.get_map().get_icon(map_node.get_icon()),
            x - 16.0,
            y - 16.0,
            WHITE,
        );
    }

    fn draw_path(&self, start: &MapNode, end: &MapNode, thicker: bool, color: Color) {
        draw_line(
            start.get_position().x * screen_width(),
            start.get_position().y * screen_height(),
            end.get_position().x * screen_width(),
            end.get_position().y * screen_height(),
            if thicker { 4. } else { 2. },
            color,
        );
    }

    fn draw_legend(&self) {
        draw_shadowbox(Rect::new(
            screen_width() * 0.8,
            screen_height() * 0.15,
            screen_width() * 0.18,
            screen_height() * 0.3,
        ));
        let mut pos = Vec2::new(screen_width() * 0.8 + 10., screen_height() * 0.15 + 40.0);
        draw_h1(&mut pos, "  Legend");
        for (index, icon) in [
            self.get_map().get_icon(&MapIcon::Boss),
            self.get_map().get_icon(&MapIcon::Enemy),
            self.get_map().get_icon(&MapIcon::Mystery),
            self.get_map().get_icon(&MapIcon::Shop),
            self.get_map().get_icon(&MapIcon::Start),
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
    }

    fn draw_stats(&self, player: &Player) {
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

    fn update_map_dragged(&mut self) {
        let (x, y) = mouse_position();
        if is_mouse_button_down(MouseButton::Middle) || is_mouse_button_down(MouseButton::Right) {
            if let Some(position) = self.last_pos {
                // Just ignore horizontal movement for now.
                let dy = y - position.y;
                self.camera_pos.y += dy;
            }
        }
        self.last_pos = Some(Vec2 { x, y });
    }

    fn update_node_clicked(&mut self, player: &mut Player) -> SceneTransition {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let map_node = self.get_map().get_map_node(player.get_map_position());
            for neig_num in map_node.get_neighbours() {
                let target = *neig_num;
                let neig = self.get_map().get_map_node(target);
                let dx = neig.get_position().x * screen_width() - x - self.camera_pos.x;
                let dy = neig.get_position().y * screen_height() - y - self.camera_pos.y;
                if (dx * dx + dy * dy).sqrt() < 26.0 {
                    player.set_map_position(target);
                    self.map
                        .get_map_nodes_mut()
                        .get_mut(target)
                        .expect("expected map_node to enter exists")
                        .mark_visited();
                    return self
                        .get_map()
                        .get_map_node(target)
                        .get_event()
                        .trigger(player);
                }
            }
        }
        SceneTransition::None
    }

    fn is_player_option(&self, map_node: &MapNode, player: &Player) -> bool {
        let player_position = self
            .get_map()
            .get_map_nodes()
            .get(player.get_map_position())
            .expect("expect exists");
        for neig in player_position.get_neighbours() {
            let neig = self.get_map().get_map_node(*neig);
            if std::ptr::eq(neig, map_node) {
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
        self.draw_background();
        self.draw_map(player);
        self.draw_legend();
        self.draw_stats(player);
    }

    fn update(&mut self, player: &mut Player) -> SceneTransition {
        self.update_map_dragged();
        self.update_node_clicked(player)
    }
}
