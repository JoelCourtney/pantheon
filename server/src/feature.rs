use crate::character::*;
use serde::Serialize;

pub trait Featured {
    fn features(&mut self) -> Vec<Feature> { vec![] }
}

#[derive(Debug, Serialize, Default)]
pub struct Feature<'a> {
    pub name: &'static str,
    pub description: &'static str,
    pub choice: Choice<'a>
}

pub type Trait<'a> = Feature<'a>;

#[derive(Debug, Serialize)]
pub enum Choice<'a> {
    Language(&'a mut Language),
    // Skill(*mut Skill),
    None
}

impl Default for Choice<'_> {
    fn default() -> Self {
        Choice::None
    }
}