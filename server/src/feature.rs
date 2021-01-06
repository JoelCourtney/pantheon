use crate::character::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Feature<'a> {
    pub name: String,
    pub description: String,
    pub choice: Choice<'a>
}

#[derive(Debug, Serialize)]
pub(crate) enum Choice<'a> {
    Language(&'a mut Language),
    // Skill(*mut Skill),
    None
}
