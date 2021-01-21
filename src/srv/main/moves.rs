use serde::Serialize;

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

#[derive(Debug, Serialize)]
pub enum Range {
    Fixed(usize),
    Extra(usize, usize)
}

#[derive(Debug, Serialize)]
pub enum CastingTime {
    Action,
    BonusAction,
    Reaction,
    Ritual(&'static str)
}

#[derive(Debug, Serialize)]
pub struct CastingComponents {
    verbal: bool,
    somatic: bool,
    material: Option<&'static str>
}