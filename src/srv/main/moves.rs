use serde::Serialize;
use crate::misc::{Range, Ability, Damage, WeaponType};

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
        name: &'static str,
        time: MoveTime,
        description: &'static str
    }
}

// #[derive(Debug, Serialize)]
// pub struct AttackMove {
//     pub name: &'static str,
//     pub time: MoveTime,
//     pub hit: i32,
//     pub damage: Damage,
//     pub range: Range,
//     pub properties: Vec<&'static str>,
//
//     #[serde(skip)]
//     pub(crate) use_modifier: Ability,
//     #[serde(skip)]
//     pub(crate) weapon_type: WeaponType
// }
//
// #[derive(Debug, Serialize)]
// pub struct CastMove {
//     pub name: &'static str,
//     pub range: Range,
//     pub time: MoveTime
// }
//
// #[derive(Debug, Serialize)]
// pub struct MiscMove {
//     pub name: &'static str,
//     pub description: &'static str,
//     pub time: MoveTime
// }

#[derive(Debug, Serialize)]
pub enum MoveTime {
    Action,
    BonusAction,
    Reaction,
    Other(&'static str)
}