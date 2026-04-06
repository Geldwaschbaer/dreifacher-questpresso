use crate::{
    dialog::{Dialog, DialogBuilder},
    entity::{
        attack::Attack,
        enemy::{Enemy, EnemyBuilder},
        player::Player,
        stat::Stat,
    },
    scene::{SceneTransition, combat_scene::CombatScene, dialog_scene::DialogScene},
};
use async_from::{AsyncFrom, async_trait};
use serde::Deserialize;

#[derive(Clone)]
pub enum Event {
    // return back to the global map
    ReturnToMap,
    // opens a new dialog
    OpenDialog(Dialog),

    EnterCombat(Enemy),

    LearnAttack(Attack),

    UpgradeStat(Stat, i32),
}

impl Event {
    pub fn trigger(&self, player: &mut Player) -> SceneTransition {
        match self {
            Event::ReturnToMap => SceneTransition::Clear,
            Event::OpenDialog(dialog) => {
                player.set_dialog_position(0);
                SceneTransition::Push(Box::new(DialogScene::new(dialog.clone())))
            }
            Event::EnterCombat(mob) => {
                SceneTransition::Push(Box::new(CombatScene::new(mob.clone())))
            }
            Event::LearnAttack(attack) => {
                player
                    .get_entity_mut()
                    .get_attacks_mut()
                    .push(attack.clone());
                SceneTransition::None
            }
            Event::UpgradeStat(stat, times) => {
                player.get_entity_mut().upgrade_stat(stat, *times);
                SceneTransition::None
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventBuilder {
    // return back to the global map
    ReturnToMap,
    // opens a new dialog
    OpenDialog(DialogBuilder),

    EnterCombat(EnemyBuilder),

    LearnAttack(Attack),

    UpgradeStat(Stat, i32),
}

#[async_trait]
impl AsyncFrom<EventBuilder> for Event {
    async fn async_from(value: EventBuilder) -> Self {
        match value {
            EventBuilder::ReturnToMap => Event::ReturnToMap,
            EventBuilder::EnterCombat(enemy) => Event::EnterCombat(Enemy::async_from(enemy).await),
            EventBuilder::OpenDialog(dialog) => Event::OpenDialog(Dialog::async_from(dialog).await),
            EventBuilder::LearnAttack(attack) => Event::LearnAttack(attack),
            EventBuilder::UpgradeStat(stat, times) => Event::UpgradeStat(stat, times),
        }
    }
}
