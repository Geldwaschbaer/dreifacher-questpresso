use crate::entity::{Entity, buff::Buff, stat::Stat};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Attack {
    description: String,
    #[serde(default = "Default::default")]
    base_damage: i32,
    #[serde(default = "Default::default")]
    base_heal: i32,
    #[serde(default = "Default::default")]
    required_mana: i32,
    scales_with: Stat,
    #[serde(default = "Default::default")]
    apply_buffs: Vec<Buff>,
    #[serde(default = "Default::default")]
    receive_buffs: Vec<Buff>,
}

impl Attack {
    pub fn get_damage(&self, user: &Entity) -> i32 {
        if self.base_damage > 0 {
            self.base_damage + user.get_stat(&self.scales_with)
        } else {
            0
        }
    }

    pub fn get_heal(&self, user: &Entity) -> i32 {
        if self.base_heal > 0 {
            self.base_heal + user.get_stat(&self.scales_with)
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

    pub fn get_applied_buffs(&self) -> &Vec<Buff> {
        &self.apply_buffs
    }

    pub fn get_received_buffs(&self) -> &Vec<Buff> {
        &self.receive_buffs
    }
}
