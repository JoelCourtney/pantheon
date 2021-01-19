use serde::{Deserialize, Serialize};

use crate::feature::{Feature, Choose};
use crate::misc::*;
use std::fmt::Debug;
use macros::{dynamic_choose, FinalizeCharacter};
use crate::content::Content;
use crate::content::common::CommonRules;
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;
use maplit::hashset;

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    name: String,

    health: usize,
    temp_health: usize,

    base_strength: usize,
    base_dexterity: usize,
    base_constitution: usize,
    base_intelligence: usize,
    base_wisdom: usize,
    base_charisma: usize,

    alignment: Alignment,

    race: Box<dyn Race>,
    classes: Vec<Box<dyn Class>>
}

impl StoredCharacter {
    pub fn resolve(&self) -> Result<FinalCharacter, ()> {
        let mut char = Character {
            name: Modifiable::new(self.name.clone()),
            health: Modifiable::new(self.health),
            temp_health: Modifiable::new(self.temp_health),

            strength: Modifiable::new(self.base_strength),
            dexterity: Modifiable::new(self.base_dexterity),
            constitution: Modifiable::new(self.base_constitution),
            intelligence: Modifiable::new(self.base_intelligence),
            wisdom: Modifiable::new(self.base_wisdom),
            charisma: Modifiable::new(self.base_charisma),

            alignment: self.alignment,

            ..Default::default()
        };

        CommonRules::declare(&mut char);
        self.race.declare(&mut char);
        for class in &self.classes {
            class.declare(&mut char);
        }

        let mut old_count: i64  = -2;
        let mut count: i64 = -1;
        let mut modify_iterations = 0;
        while count != 0 && old_count != count {
            old_count = count;

            CommonRules::modify(&mut char);
            self.race.modify(&mut char);
            for class in &self.classes {
                class.modify(&mut char);
            }

            count = char.count_unresolved().into();

            modify_iterations += 1;
        }
        dbg!(modify_iterations);
        if count != 0 {
            dbg!(&char);
            println!("modifier deadlock");
            // Err(TODO("make an error for this"))
            Err(())
        } else {
            char.race_traits.extend(self.race.features());
            char.feats.extend(self.race.feats());
            for class in &self.classes {
                char.class_features.extend(class.features());
                char.feats.extend(class.feats());
            }
            // char.background_features
            Ok(char.finalize())
        }
    }
}

#[derive(Debug, Default, FinalizeCharacter)]
pub struct Character {
    pub name: Modifiable<String>,

    // HEALTH
    pub health: Modifiable<usize>,
    pub temp_health: Modifiable<usize>,
    pub max_health: Modifiable<usize>,

    // ABILITIES
    pub strength: Modifiable<usize>,
    pub dexterity: Modifiable<usize>,
    pub constitution: Modifiable<usize>,
    pub intelligence: Modifiable<usize>,
    pub wisdom: Modifiable<usize>,
    pub charisma: Modifiable<usize>,

    pub strength_modifier: Modifiable<i32>,
    pub dexterity_modifier: Modifiable<i32>,
    pub constitution_modifier: Modifiable<i32>,
    pub intelligence_modifier: Modifiable<i32>,
    pub wisdom_modifier: Modifiable<i32>,
    pub charisma_modifier: Modifiable<i32>,

    // INITIATIVE
    pub initiative: Modifiable<i32>,

    // SIZE
    pub size: Modifiable<CreatureSize>,


    // PROFICIENCIES AND LANGUAGES
    pub skill_proficiencies: Modifiable<Vec<(Skill, ProficiencyType)>>,
    pub tool_proficiencies: Modifiable<Vec<(&'static str, ProficiencyType)>>,
    pub languages: Modifiable<Vec<Language>>,

    // SPEED
    pub walking_speed: Modifiable<usize>,
    pub flying_speed: Modifiable<usize>,
    pub climbing_speed: Modifiable<usize>,
    pub swimming_speed: Modifiable<usize>,
    pub burrowing_speed: Modifiable<usize>,

    // NOTES
    pub saving_throw_notes: Modifiable<Vec<&'static str>>,

    // DO NOT MODIFY FIELDS AFTER THIS POINT

    // FEATURES, TRAITS, AND FEATS
    race_traits: Vec<Feature>,
    class_features: Vec<Feature>,
    background_features: Vec<Feature>,
    feats: Vec<Vec<Feature>>,

    // ALIGNMENT
    alignment: Alignment,
}

#[derive(Default, Debug)]
pub struct Modifiable<T>
    where T: Default + Debug + Serialize {
    value: T,
    initializers: HashSet<&'static str>,
    modifiers: HashSet<&'static str>,
    finalizers: HashSet<&'static str>
}

impl<T> Modifiable<T>
    where T: Serialize + Default + Debug {

    fn new(v: T) -> Self {
        Modifiable {
            value: v,
            initializers: hashset! {},
            modifiers: hashset! {},
            finalizers: hashset! {}
        }
    }
    pub fn unwrap(self) -> T {
        self.value
    }

    pub fn declare_initializer(&mut self, who: &'static str) {
        self.initializers.insert(who);
    }
    pub fn declare_modifier(&mut self, who: &'static str) {
        self.modifiers.insert(who);
    }
    pub fn declare_finalizer(&mut self, who: &'static str) {
        self.finalizers.insert(who);
    }

    pub fn initialized(&self) -> bool {
        self.initializers.is_empty()
    }
    pub fn modified(&self) -> bool {
        self.initializers.is_empty() && self.modifiers.is_empty()
    }
    pub fn finalized(&self) -> bool {
        self.initializers.is_empty() && self.modifiers.is_empty() && self.finalizers.is_empty()
    }

    pub fn initialize(&mut self, who: &'static str) -> bool {
        self.initializers.remove(who)
    }
    pub fn modify(&mut self, who: &'static str) -> bool {
        self.initializers.is_empty() && self.modifiers.remove(who)
    }
    pub fn finalize(&mut self, who: &'static str) -> bool {
        self.initializers.is_empty() && self.modifiers.is_empty() && self.finalizers.remove(who)
    }

    pub fn count_unresolved(&self) -> u32 {
        (self.initializers.len() + self.modifiers.len() + self.finalizers.len()) as u32
    }
}

impl<T> Deref for Modifiable<T>
    where T: Debug + Default + Serialize {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Modifiable<T>
    where T: Debug + Default + Serialize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[dynamic_choose]
pub trait Race: Content + Debug {
    fn content_name(&self) -> &'static str;
}

#[dynamic_choose]
pub trait Class: Content + Debug {
    fn content_name(&self) -> &'static str;
}

#[dynamic_choose]
pub trait Feat: Content + Debug {
    fn content_name(&self) -> &'static str;
}