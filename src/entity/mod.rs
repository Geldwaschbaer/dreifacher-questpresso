pub mod attack;
pub mod buff;
pub mod enemy;
pub mod player;
pub mod stat;

use async_from::{AsyncFrom, async_trait};
use macroquad::texture::{Texture2D, load_texture};
use serde::Deserialize;

use crate::entity::{attack::Attack, buff::Buff, stat::Stat};

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
    buffs: Vec<Buff>,
    texture: Texture2D,
    lost_hp: bool,
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
            buffs: Vec::new(),
            texture,
            lost_hp: false,
        }
    }

    pub fn use_attack(&mut self, attack: usize, target: &mut Entity) {
        let attack = self.attacks.get(attack).expect("expected attack exists");
        if self.mana >= attack.get_required_mana() {
            for buff in attack.get_applied_buffs() {
                target.buffs.push(buff.clone());
            }
            for buff in attack.get_received_buffs() {
                self.buffs.push(buff.clone())
            }
            let mut damage = attack.get_damage(self);
            let mut heal = attack.get_heal(self);
            for buff in &self.buffs {
                damage = buff.translate_damage_applied(damage);
                heal = buff.translate_heal_received(heal);
            }
            for buff in &target.buffs {
                damage = buff.translate_damage_received(damage);
            }
            target.hit_points = (target.hit_points - damage).max(0);
            if damage > 0 {
                target.lost_hp = true;
            }
            self.hit_points = (self.hit_points + heal).min(self.constitution * 5);
            self.mana -= attack.get_required_mana();
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

    pub fn end_turn(&mut self) {
        for buff in self.buffs.clone() {
            buff.end_of_turn(self);
        }
        self.buffs.clear();
        self.lost_hp = false;
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

    pub fn restore_hp(&mut self, hp: i32) {
        self.hit_points = (self.hit_points + hp).min(self.constitution * 5);
    }

    pub fn restore_mp(&mut self, mp: i32) {
        self.mana = (self.hit_points + mp).min(self.intelligence * 3);
    }

    pub fn get_stat(&self, stat: &Stat) -> i32 {
        match stat {
            Stat::Str => self.strength,
            Stat::Dex => self.dexterity,
            Stat::Con => self.constitution,
            Stat::Int => self.intelligence,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    pub fn has_lost_hp(&self) -> bool {
        self.lost_hp
    }

    pub fn get_attacks(&self) -> &Vec<Attack> {
        &self.attacks
    }

    pub fn get_attacks_mut(&mut self) -> &mut Vec<Attack> {
        &mut self.attacks
    }

    pub fn get_buffs(&self) -> &Vec<Buff> {
        &self.buffs
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
        Entity {
            name: value.name,
            hit_points: value.constitution * 5,
            mana: value.intelligence * 3,
            strength: value.strength,
            dexterity: value.dexterity,
            constitution: value.constitution,
            intelligence: value.intelligence,
            attacks: value.attacks,
            buffs: Vec::new(),
            texture,
            lost_hp: false,
        }
    }
}
