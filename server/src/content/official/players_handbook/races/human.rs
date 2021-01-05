macros::register!(("Human", "Race"));

use crate::modify::*;
use crate::character::*;
use crate::feature::*;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Debug,Default)]
pub struct Human {
    pub(crate) extra_language: Language
}

impl Race for Human {
    fn traits(&mut self) -> Vec<Feature> {
        vec![
            Feature {
                name: "Age".to_string(),
                description: "Humans reach adulthood in their late teens and live less than a century.".to_string(),
                choice: Choice::None
            },
            Feature {
                name: "Languages".to_string(),
                description: "You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.".to_string(),
                choice: Choice::Language(&mut self.extra_language)
            }
        ]
    }
}

impl Modify for Human {
    fn initialize(&self, c: &mut Character) {
        c.size = CreatureSize::Medium;
        c.walking_speed = 30;

        c.proficiencies.push(Proficiency::Language(Language::Common));
    }
    fn modify(&self, c: &mut Character) {
        c.strength += 1;
        c.dexterity += 1;
        c.constitution += 1;
        c.intelligence += 1;
        c.wisdom += 1;
        c.charisma += 1;
    }
}
