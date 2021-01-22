use serde::Serialize;
use crate::misc::Range;

#[derive(Debug, Serialize)]
pub enum Move {
    AttackMove {
        name: &'static str,
        hit: isize,
        damage: usize,
        range: Range,
        properties: Vec<&'static str>
    },
    CastMove {
        name: &'static str,
        range: Range,
    },
    Other {
        name: &'static str,
        description: &'static str
    }
}

