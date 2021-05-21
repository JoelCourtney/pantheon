use serde::Serialize;
use crate::misc::{Range, Ability, Damage, WeaponType};
use crate::ui::Element;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Move {
    Attack {
        name: &'static str,
        time: MoveTime,
        hit: i32,
        damage: Damage,
        range: Range,
        properties: Vec<&'static str>,

        #[serde(skip)]
        use_modifier: Ability,
        #[serde(skip)]
        weapon_type: WeaponType
    },
    Cast {
        name: &'static str,
        time: MoveTime,
        range: Range,
    },
    Other {
        element: Element,
        time: MoveTime,
    }
}

#[derive(Debug, Serialize)]
pub enum MoveTime {
    Action,
    BonusAction,
    Reaction,
    Other(&'static str)
}