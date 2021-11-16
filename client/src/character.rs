use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use seed::log;
use serde::{Serialize, Deserialize};

/// Version of the Character struct that is stored as a json file for saving.
///
/// It contains the minimal amount of information to completely reconstruct the character.
/// Much of the information is stored in the content structs (like races, classes, etc),
/// and the content-creator should also try to be as minimal as possible. (de)Serialization
/// of the nested content information is handled automatically by Serde and Typetag.
#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    pub(crate) name: String,

    pub(crate) health: u32,
    pub(crate) temp_health: u32,

    // pub(crate) base_abilities: AbilityMap<u32>,

    // alignment: Alignment,

    inspiration: bool,

    // money: MoneyTypeMap<u32>,

    // pub(crate) race: Box<dyn Race>,
    // pub(crate) classes: Vec<(Box<dyn Class>, u32)>,
    // pub(crate) background: Box<dyn Background>,

    // inventory: Vec<(Box<dyn Item>, Equipped, bool)>,

    pub(crate) description: String
}

pub fn try_it() {
    let char = DummyCharacter {
        x: CharacterValue(RefCell::new(Staged {
            value: "hello".to_string(),
            ops: Some(vec![
                Box::new(
                    |x, char| {
                        log!("evaling x");
                        x.push_str(char.y.evaluate(char).get())
                    }
                )
            ])
        })),
        y: CharacterValue(RefCell::new(Staged {
            value: "".to_string(),
            ops: Some(vec![
                Box::new(
                    |y, _| {
                        log!("evaling y");
                        *y = " world!".to_string()
                    }
                )
            ])
        }))
    };
    log!(char.x.evaluate(&char).get());
    log!(char.x.evaluate(&char).get());
}

struct DummyCharacter {
    x: CharacterValue<String>,
    y: CharacterValue<String>
}

#[repr(transparent)]
struct CharacterValue<T>(RefCell<Staged<T>>);

struct ValueGuard<'a, T>(Ref<'a, T>);

impl<T> ValueGuard<'_, T> {
    fn get(&self) -> &T {
        self.0.deref()
    }
}

impl<T> CharacterValue<T> {
    fn evaluate(&self, character: &DummyCharacter) -> ValueGuard<T> {
        self.0.borrow_mut().evaluate(character);
        ValueGuard(Ref::map(self.0.borrow(), |staged| {
            staged.get()
        }))
    }
}

struct Staged<T> {
    value: T,
    ops: Option<Vec<Box<dyn FnOnce(&mut T, &DummyCharacter)>>>
}

impl<T> Staged<T> {
    fn evaluate(&mut self, character: &DummyCharacter) {
        if let Some(ops) = self.ops.take() {
            for op in ops {
                op(&mut self.value, character);
            }
        }
    }

    fn get(&self) -> &T {
        &self.value
    }
}