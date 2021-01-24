use serde::Serialize;
use crate::misc::{Range, Ability, Damage};

#[derive(Debug, Serialize)]
pub struct AttackMove {
    pub name: &'static str,
    pub time: MoveTime,
    pub hit: i32,
    pub damage: Damage,
    pub range: Range,
    pub properties: Vec<&'static str>,

    #[serde(skip)]
    pub(crate) use_modifier: Ability
}

#[derive(Debug, Serialize)]
pub struct CastMove {
    pub name: &'static str,
    pub range: Range,
    pub time: MoveTime
}

#[derive(Debug, Serialize)]
pub struct MiscMove {
    pub name: &'static str,
    pub description: &'static str,
    pub time: MoveTime
}

#[derive(Debug, Serialize)]
pub enum MoveTime {
    Action,
    BonusAction,
    Reaction,
    Other
}