pub mod enemy;
pub mod player;

use async_from::{AsyncFrom, async_trait};
use macroquad::texture::{FilterMode, Texture2D, load_texture};
use serde::Deserialize;

#[derive(Clone)]
pub struct Entity {
    name: String,
    hit_points: i32,
    mana: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    attacks: Vec<Attack>,
    texture: Texture2D,
}

impl Entity {
    pub fn new(name: String, texture: Texture2D) -> Entity {
        Entity {
            name,
            hit_points: 5,
            mana: 3,
            strength: 1,
            dexterity: 1,
            constitution: 1,
            intelligence: 1,
            attacks: Vec::new(),
            texture,
        }
    }

    pub fn use_attack(&mut self, attack: usize, target: &mut Entity) {
        let attack = self.attacks.get(attack).expect("expected attack exists");
        if self.mana >= attack.required_mana {
            target.hit_points -= attack.get_damage(self);
            self.hit_points = (self.hit_points + attack.get_heal(self)).min(self.constitution * 5);
            self.mana -= attack.required_mana;
        }
    }

    pub fn upgrade_stat(&mut self, stat: &Stat, times: i32) {
        match stat {
            Stat::Con => {
                self.constitution += times;
                self.hit_points += 5 * times;
            }
            Stat::Dex => self.dexterity += times,
            Stat::Int => {
                self.intelligence += times;
                self.mana += 3 * times;
            }
            Stat::Str => self.strength += times,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn get_hp(&self) -> (i32, i32) {
        (self.hit_points, self.constitution * 5)
    }

    pub fn get_mp(&self) -> (i32, i32) {
        (self.mana, self.intelligence * 3)
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    pub fn get_attacks(&self) -> &Vec<Attack> {
        &self.attacks
    }

    pub fn get_attacks_mut(&mut self) -> &mut Vec<Attack> {
        &mut self.attacks
    }
}

#[derive(Deserialize)]
pub struct EntityBuilder {
    name: String,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    attacks: Vec<Attack>,
    texture: String,
}

#[async_trait]
impl AsyncFrom<EntityBuilder> for Entity {
    async fn async_from(value: EntityBuilder) -> Self {
        let texture = load_texture(&value.texture).await.expect("texture exists");
        texture.set_filter(FilterMode::Nearest);
        Entity {
            name: value.name,
            hit_points: value.constitution * 5,
            mana: value.intelligence * 3,
            strength: value.strength,
            dexterity: value.dexterity,
            constitution: value.constitution,
            intelligence: value.intelligence,
            attacks: value.attacks,
            texture,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Attack {
    description: String,
    base_damage: i32,
    base_heal: i32,
    required_mana: i32,
    scales_with: Stat,
}

impl Attack {
    pub fn get_damage(&self, user: &Entity) -> i32 {
        if self.base_damage > 0 {
            self.base_damage + self.scales_with.get_bonus(user)
        } else {
            0
        }
    }

    pub fn get_heal(&self, user: &Entity) -> i32 {
        if self.base_heal > 0 {
            self.base_heal + self.scales_with.get_bonus(user)
        } else {
            0
        }
    }

    pub fn get_required_mana(&self) -> i32 {
        self.required_mana
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone, Deserialize)]
pub enum Stat {
    Str,
    Dex,
    Con,
    Int,
}

impl Stat {
    pub fn get_bonus(&self, entity: &Entity) -> i32 {
        match self {
            Self::Str => entity.strength,
            Self::Dex => entity.dexterity,
            Self::Con => entity.constitution,
            Self::Int => entity.intelligence,
        }
    }
}
