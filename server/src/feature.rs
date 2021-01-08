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
    pub choose: Box<dyn Choice + 'a>
}

pub type Trait<'a> = Feature<'a>;

pub trait Choice: Debug {
    fn choices(&self) -> Vec<&'static str>;
    fn choose(&mut self, choice: &str, index: usize);
}

impl Default for Box<dyn Choice> {
    fn default() -> Self {
        Box::new(
            EmptyChoice {}
        )
    }
}

#[derive(Debug)]
pub struct EmptyChoice;
impl Choice for EmptyChoice {
    fn choices(&self) -> Vec<&'static str> { vec![] }
    fn choose(&mut self, _choice: &str, _index: usize) {}
}

pub trait Choose {
    fn choose<'a>(_: &'a mut Self) -> Box<dyn Choice + 'a>;
    fn choose_multiple<'a>(_: Vec<&'a mut Self>) -> Box<dyn Choice + 'a>;
}
