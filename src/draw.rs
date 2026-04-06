use crate::entity::{Attack, Entity, player::Player};
use macroquad::prelude::*;
use std::iter::Iterator;

pub const ACTIVATED: Color = Color::from_hex(0x1b252e);
pub const AVAILABLE: Color = Color::from_hex(0x585858);
pub const TEXT_COL: Color = Color::from_hex(0x00AA00);
pub const HEADER_COL: Color = Color::from_hex(0x55FF55);
pub const BACKGROUND: Color = Color::from_hex(0x000000);

pub fn draw_lifebar(offset: &mut Vec2, entity: &Entity) {
    draw_shadowbox(Rect::new(
        screen_width() * 0.05 + offset.x,
        screen_height() * 0.05 + offset.y,
        screen_width() * 0.3,
        screen_height() * 0.1,
    ));

    let mut pos = Vec2::new(
        screen_width() * 0.05 + offset.x + 10.0,
        screen_height() * 0.07 + offset.y,
    );
    draw_p(&mut pos, &entity.get_name().to_uppercase());

    let health = entity.get_hp();
    let mana = entity.get_mp();
    draw_p(
        &mut pos,
        &format!("hp: {}/{}, mp: {}/{}", health.0, health.1, mana.0, mana.1),
    );

    draw_shadowbox_ex(
        Rect::new(
            screen_width() * 0.18 + offset.x,
            screen_height() * 0.11 + offset.y,
            screen_width() * 0.15,
            25.,
        ),
        DrawShadowboxParams {
            padding: Rect::new(2.0, 2.0, 2.0, 3.0),
            ..Default::default()
        },
    );
    draw_rectangle(
        screen_width() * 0.18 + offset.x,
        screen_height() * 0.11 + offset.y,
        screen_width() * 0.15 * health.0 as f32 / health.1 as f32,
        25.,
        PINK,
    );
}

pub fn draw_shadowbox(rect: Rect) {
    draw_shadowbox_ex(rect, DrawShadowboxParams::default());
}

pub fn draw_h1(pos: &mut Vec2, text: &str) {
    draw_text(&text.to_uppercase(), pos.x, pos.y, 30.0, HEADER_COL);
    pos.y += 30.;
}

pub fn draw_p(pos: &mut Vec2, text: &str) {
    draw_p_ex(pos, text, Default::default());
}

pub struct DrawParagraphParams {
    pub font_size: f32,
    pub color: Color,
    pub split_line: bool,
    pub margin: Rect,
}

impl Default for DrawParagraphParams {
    fn default() -> DrawParagraphParams {
        DrawParagraphParams {
            font_size: 22.,
            color: TEXT_COL,
            split_line: true,
            margin: Rect::new(0., 0., 0., 0.),
        }
    }
}

pub fn draw_p_ex(pos: &mut Vec2, text: &str, params: DrawParagraphParams) {
    if params.split_line {
        for line in text.split("\n") {
            draw_text(
                line,
                pos.x + params.margin.x,
                pos.y + params.margin.y,
                params.font_size,
                params.color,
            );
            pos.y += params.font_size + params.margin.y + params.margin.h;
        }
    } else {
        draw_text(
            text,
            pos.x + params.margin.x,
            pos.y + params.margin.y,
            params.font_size,
            params.color,
        );
        pos.y += params.font_size + params.margin.y + params.margin.h;
    }
}

pub fn draw_ol<'a, I>(pos: &mut Vec2, items: impl Iterator<Item = I>)
where
    I: Into<&'a str>,
{
    for (index, item) in items.enumerate() {
        draw_text(&format!("{}. ", index + 1), pos.x - 10., pos.y, 22., TEXT_COL);
        draw_p_ex(
            pos,
            item.into(),
            DrawParagraphParams {
                split_line: true,
                margin: Rect::new(20.0, 0., 0., 0.),
                ..Default::default()
            },
        );
    }
}

pub fn draw_attacks<'a>(pos: &mut Vec2, player: &Player, items: impl Iterator<Item = &'a Attack>) {
    for (index, item) in items.enumerate() {
        draw_text(&format!("{}. ", index + 1), pos.x - 10., pos.y, 22., TEXT_COL);
        draw_p_ex(
            pos,
            item.get_description(),
            DrawParagraphParams {
                split_line: true,
                margin: Rect::new(20.0, 0., 0., 20.),
                ..Default::default()
            },
        );
        let mut x = pos.x + 25.;
        if item.get_damage(player.get_entity()) > 0 {
            draw_text(
                &format!("damage: {}", item.get_damage(player.get_entity())),
                x,
                pos.y - 20.,
                22.,
                RED,
            );
            x += 150.
        }
        if item.get_heal(player.get_entity()) > 0 {
            draw_text(
                &format!("heal: {}", item.get_heal(player.get_entity())),
                x,
                pos.y - 20.,
                22.,
                GREEN,
            );
            x += 150.
        }
        if item.get_required_mana() > 0 {
            draw_text(
                &format!("mana: {}", item.get_required_mana()),
                x,
                pos.y - 20.,
                22.,
                BLUE,
            );
        }
    }
}

pub struct DrawShadowboxParams {
    pub padding: Rect,
    pub fill: Color,
    pub stroke: Color,
}

impl Default for DrawShadowboxParams {
    fn default() -> DrawShadowboxParams {
        DrawShadowboxParams {
            padding: Rect::new(2.0, 2.0, 2.0, 10.0),
            fill: BACKGROUND,
            stroke: HEADER_COL,
        }
    }
}

pub fn draw_shadowbox_ex(rect: Rect, params: DrawShadowboxParams) {
    let padding = params.padding;
    draw_rectangle(
        rect.x - padding.x,
        rect.y - padding.y,
        rect.w + padding.x + padding.w,
        rect.h + padding.y + padding.h,
        params.stroke,
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, params.fill);
}
