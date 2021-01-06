use serde::Serialize;
use std::fmt::Debug;

pub trait Featured {
    fn features(&mut self) -> Vec<Feature> { vec![] }
}

#[derive(Debug, Serialize, Default)]
pub struct Feature<'a> {
    pub name: &'static str,
    pub description: &'static str,

    #[serde(skip)]
    pub choice: Box<dyn Choice + 'a>
}

impl Default for Box<dyn Choice> {
    fn default() -> Self {
        Box::new(
            EmptyChoice {}
        )
    }
}

pub type Trait<'a> = Feature<'a>;

pub trait Choice: Debug {
    fn choices(&self) -> Vec<&str> { vec![] }
    fn choose(&mut self, _choice: &str) {}
}

pub trait Choose {
    fn choice<'a>(_: &'a mut Self) -> Box<dyn Choice + 'a>;
}

#[derive(Debug)]
pub struct EmptyChoice;
impl Choice for EmptyChoice {}