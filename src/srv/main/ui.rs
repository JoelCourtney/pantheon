use serde::Serializer;
use std::fmt::Debug;
use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum Element {
    Text(&'static str),
    Choice {
        text: &'static str,
        data: *mut dyn Choose,
        unique: bool
    },
    Resource {
        text: &'static str,
        data: *mut u32,
        max: u32
    },
    Trigger {
        text: &'static str,
        event: Event
    }
}

impl serde::Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut state;
        match self {
            Element::Text(text) => {
                state = serializer.serialize_struct("Element", 2)?;
                state.serialize_field("type", "text")?;
                state.serialize_field("text", text)?;
            }
            Element::Choice { text, data, unique } => unsafe {
                state = serializer.serialize_struct("Element", 4)?;
                state.serialize_field("type", "choice")?;
                state.serialize_field("text", text)?;
                state.serialize_field("data", &(**data).to_choice())?;
                state.serialize_field("unique", unique)?;
            }
            Element::Resource { text, data, max } => unsafe {
                state = serializer.serialize_struct("Element", 4)?;
                state.serialize_field("type", "resource")?;
                state.serialize_field("text", text)?;
                state.serialize_field("data", &**data)?;
                state.serialize_field("unique", max)?;
            }
            Element::Trigger { text, event } => {
                state = serializer.serialize_struct("Element", 3)?;
                state.serialize_field("type", "trigger")?;
                state.serialize_field("text", text)?;
                state.serialize_field("event", event)?;
            }
        }
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    LongRest,
    ShortRest,
    Custom(&'static str)
}

#[derive(Serialize, Clone)]
pub struct ChoiceSerial {
    pub current_choices: Vec<&'static str>,
    pub all_choices: Vec<&'static str>
}

impl ChoiceSerial {
    pub fn from_vecs(current_choices: Vec<&'static str>, all_choices: Vec<&'static str>) -> ChoiceSerial {
        ChoiceSerial {
            current_choices,
            all_choices
        }
    }
}

pub trait Choose: Debug {
    fn choose(&mut self, choice: &str, index: usize);
    fn to_choice(&self) -> ChoiceSerial;
}