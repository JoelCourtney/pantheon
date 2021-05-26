use serde::Serializer;
use std::fmt::Debug;
use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum Element {
    Str(&'static str),
    String(String),
    Choice {
        text: &'static str,
        data: *mut dyn Chooseable,
        unique: bool
    },
    Resource {
        text: &'static str,
        data: *mut u32,
        max: u32
    },
    Trigger {
        text: &'static str,
        event: Event,
        button: &'static str
    },
    Toggle {
        text: &'static str,
        data: *mut dyn Toggleable,
        button: Vec<&'static str>
    }
}

impl serde::Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut state;
        match self {
            Element::Str(text) => {
                state = serializer.serialize_struct("Element", 2)?;
                state.serialize_field("type", "text")?;
                state.serialize_field("text", text)?;
            }
            Element::String(text) => {
                state = serializer.serialize_struct("Element", 2)?;
                state.serialize_field("type", "text")?;
                state.serialize_field("text", text)?;
            }
            Element::Choice { text, data, unique } => unsafe {
                state = serializer.serialize_struct("Element", 4)?;
                state.serialize_field("type", "choice")?;
                state.serialize_field("text", text)?;
                state.serialize_field("data", &(**data).to_serial())?;
                state.serialize_field("unique", unique)?;
            }
            Element::Resource { text, data, max } => unsafe {
                state = serializer.serialize_struct("Element", 4)?;
                state.serialize_field("type", "resource")?;
                state.serialize_field("text", text)?;
                state.serialize_field("data", &**data)?;
                state.serialize_field("unique", max)?;
            }
            Element::Trigger { text, event, button } => {
                state = serializer.serialize_struct("Element", 4)?;
                state.serialize_field("type", "trigger")?;
                state.serialize_field("text", text)?;
                state.serialize_field("event", event)?;
                state.serialize_field("button", button)?;
            }
            Element::Toggle { text, button, ..} => {
                state = serializer.serialize_struct("Element", 3)?;
                state.serialize_field("type", "toggle")?;
                state.serialize_field("text", text)?;
                state.serialize_field("button", button)?;
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

pub trait Chooseable: Debug {
    fn choose(&mut self, choice: &str, index: usize);
    fn to_serial(&self) -> ChoiceSerial;
}

pub trait Toggleable {
    fn toggle(&mut self, index: usize);
}

impl Toggleable for bool {
    fn toggle(&mut self, index: usize) {
        if index == 0 {
            *self = !*self;
        } else {
            panic!("toggle index out of bounds: {}. length is 1", index)
        }
    }
}

impl<const N: usize> Toggleable for [bool; N] {
    fn toggle(&mut self, index: usize) {
        if index < N {
            self[index] = !self[index]
        } else {
            panic!("toggle index out of bounds: {}. length is {}", index, N)
        }
    }
}

impl Toggleable for Vec<bool> {
    fn toggle(&mut self, index: usize) {
        if index < self.len() {
            self[index] = !self[index]
        } else {
            panic!("toggle index out of bounds: {}. length is {}", index, self.len())
        }
    }
}