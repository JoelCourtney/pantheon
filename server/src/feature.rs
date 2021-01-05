use crate::character::*;
use serde::Serialize;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct Feature {
    pub name: String,
    pub description: String,
    pub choice: Choice
}

#[derive(Debug)]
pub(crate) enum Choice {
    Language(*mut Language),
    Skill(*mut Skill),
    None
}
