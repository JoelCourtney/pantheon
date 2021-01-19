use serde::{Serialize, Serializer};
use std::fmt::Debug;
use serde::ser::SerializeStruct;

#[derive(Debug, Default, Serialize)]
pub struct Feature<'a>(pub &'static str, pub Option<&'a mut dyn Choose>);

pub struct ChoiceSerial {
    pub current_choices: Vec<&'static str>,
    pub all_choices: Vec<Vec<&'static str>>
}

pub trait Choose: Debug {
    fn choose(&mut self, choice: &str, index: usize);
    fn to_choice(&self, unique: bool) -> ChoiceSerial;
}

impl<'a> Serialize for &'a mut dyn Choose {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let serial = self.to_choice(false);
        let mut state = serializer.serialize_struct("ChoiceSerial", 2)?;
        state.serialize_field("current_choices", &serial.current_choices)?;
        state.serialize_field("all_choices", &serial.all_choices)?;
        state.end()
    }
}
