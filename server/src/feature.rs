use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize, Default)]
pub struct Feature(pub &'static str, pub Option<Choice>);

#[derive(Debug, Serialize, Default)]
pub struct Choice {
    pub current_choices: Vec<&'static str>,
    pub all_choices: Vec<Vec<&'static str>>
}

pub trait Choose {
    fn choose(&mut self, choice: &str, index: usize);
    fn to_choice(&self, unique: bool) -> Choice;
}
