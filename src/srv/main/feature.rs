use serde::{Serialize, Serializer};
use std::fmt::Debug;
use serde::ser::SerializeTupleStruct;

#[derive(Debug)]
pub struct Feature(pub &'static str, pub Choice);

impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut state = serializer.serialize_tuple_struct("FeatureSerial", 2)?;
        state.serialize_field(&self.0)?;
        let choice_serial: Option<ChoiceSerial> = {
            match self.1 {
                Choice::Any(c) => unsafe {
                    Some((*c).to_choice(false))
                }
                Choice::Unique(c) => unsafe {
                    Some((*c).to_choice(true))
                }
                Choice::Empty => None
            }
        };
        state.serialize_field(&choice_serial)?;
        state.end()
    }
}

#[derive(Serialize)]
pub struct ChoiceSerial {
    pub current_choices: Vec<&'static str>,
    pub all_choices: Vec<Vec<&'static str>>
}

impl ChoiceSerial {
    pub fn from_vecs(current_choices: Vec<&'static str>, all_choices: Vec<&'static str>, unique: bool) -> ChoiceSerial {
        ChoiceSerial {
            current_choices: current_choices.clone(),
            all_choices: (0..current_choices.len()).map(
                |i| {
                    if !unique {
                        all_choices.clone()
                    } else {
                        all_choices.clone()
                            .into_iter()
                            .filter(|v| v == &current_choices[i] || !current_choices.contains(&v))
                            .collect()
                    }
                }
            ).collect()
        }
    }
}

pub trait Choose: Debug {
    fn choose(&mut self, choice: &str, index: usize);
    fn to_choice(&self, unique: bool) -> ChoiceSerial;
}

#[derive(Debug)]
pub enum Choice {
    Any(*mut dyn Choose),
    Unique(*mut dyn Choose),
    Empty
}

impl Default for Choice {
    fn default() -> Self { Choice::Empty }
}