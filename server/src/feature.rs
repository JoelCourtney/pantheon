use serde::{Serialize, Serializer};
use std::fmt::Debug;
use serde::ser::SerializeTupleStruct;

#[derive(Debug, Default)]
pub struct Feature(pub &'static str, pub Option<*mut dyn Choose>);

impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut state = serializer.serialize_tuple_struct("FeatureSerial", 2)?;
        state.serialize_field(&self.0)?;
        let choice_serial: Option<ChoiceSerial> = {
            match self.1 {
                Some(c) => unsafe {
                    Some((*c).to_choice(false))
                }
                None => None
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

pub trait Choose: Debug {
    fn choose(&mut self, choice: &str, index: usize);
    fn to_choice(&self, unique: bool) -> ChoiceSerial;
}
