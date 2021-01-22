use serde::{Deserialize, Serialize};

use crate::feature::Feature;
use crate::misc::*;
use std::fmt::Debug;
use macros::FinalizeCharacter;
use crate::content::common::common_rules;
use crate::content::traits::{Race, Class};
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;
use maplit::hashset;
use crate::moves::Move;

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    pub(crate) name: String,

    pub(crate) health: usize,
    pub(crate) temp_health: usize,

    base_strength: usize,
    base_dexterity: usize,
    base_constitution: usize,
    base_intelligence: usize,
    base_wisdom: usize,
    base_charisma: usize,

    alignment: Alignment,

    inspiration: bool,

    money: [usize; 5],

    race: Box<dyn Race>,
    classes: Vec<Box<dyn Class>>
}

impl StoredCharacter {
    pub fn read(path: &str) -> StoredCharacter {
        let json = std::fs::read_to_string(path).expect(&format!("READING FAILED: {}", path));
        serde_json::from_str(&json).expect("DESERIALIZATION FAILED")
    }
    pub fn write(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self).expect("SERIALIZATION FAILED");
        std::fs::write(path, json).expect(&format!("WRITING FAILED: {}", path));
    }
    pub fn resolve(&mut self) -> Result<FinalCharacter, ()> {
        let mut char = Character {
            name: Staged::new(self.name.clone()),
            health: Staged::new(self.health),
            temp_health: Staged::new(self.temp_health),

            strength: Staged::new(self.base_strength),
            dexterity: Staged::new(self.base_dexterity),
            constitution: Staged::new(self.base_constitution),
            intelligence: Staged::new(self.base_intelligence),
            wisdom: Staged::new(self.base_wisdom),
            charisma: Staged::new(self.base_charisma),

            money: self.money,
            inspiration: self.inspiration,

            alignment: self.alignment,

            ..Default::default()
        };

        common_rules::declare(&mut char);
        self.race.declare(&mut char);
        for class in &self.classes {
            class.declare(&mut char);
        }

        let mut old_count: i64  = -2;
        let mut count: i64 = -1;
        // let mut iterations = 0;
        while count != 0 && old_count != count {
            old_count = count;

            common_rules::iterate(&mut char);
            self.race.iterate(&mut char);
            for class in &self.classes {
                class.iterate(&mut char);
            }

            count = char.count_unresolved().into();

            // iterations += 1;
        }
        // dbg!(iterations);
        if count != 0 {
            dbg!(&char);
            println!("modifier deadlock");
            // Err(TODO("make an error for this"))
            Err(())
        } else {
            common_rules::last(&mut char);
            self.race.last(&mut char);
            for class in &mut self.classes {
                class.last(&mut char);
            }
            Ok(char.finalize())
        }
    }
}

#[derive(Debug, Default, FinalizeCharacter)]
pub struct Character {
    pub name: Staged<String>,
    pub total_level: Staged<usize>,
    pub race_name: Staged<String>,
    pub class_names: Staged<Vec<String>>,

    // PROFICIENCY BONUS
    pub proficiency_bonus: Staged<usize>,

    // HEALTH
    pub health: Staged<usize>,
    pub temp_health: Staged<usize>,
    pub max_health: Staged<usize>,

    // ABILITIES
    pub strength: Staged<usize>,
    pub dexterity: Staged<usize>,
    pub constitution: Staged<usize>,
    pub intelligence: Staged<usize>,
    pub wisdom: Staged<usize>,
    pub charisma: Staged<usize>,

    pub strength_modifier: Staged<i32>,
    pub dexterity_modifier: Staged<i32>,
    pub constitution_modifier: Staged<i32>,
    pub intelligence_modifier: Staged<i32>,
    pub wisdom_modifier: Staged<i32>,
    pub charisma_modifier: Staged<i32>,

    // INITIATIVE
    pub initiative: Staged<i32>,

    // SIZE
    pub size: Staged<CreatureSize>,

    // PROFICIENCIES AND LANGUAGES
    pub skill_proficiencies: Staged<Vec<(Skill, ProficiencyType)>>,
    pub tool_proficiencies: Staged<Vec<(&'static str, ProficiencyType)>>,
    pub languages: Staged<Vec<Language>>,

    // SPEED
    pub walking_speed: Staged<usize>,
    pub flying_speed: Staged<usize>,
    pub climbing_speed: Staged<usize>,
    pub swimming_speed: Staged<usize>,
    pub burrowing_speed: Staged<usize>,

    // ATTACKS PER ACTION
    pub attacks_per_action: Staged<usize>,

    // NOTES
    pub saving_throw_notes: Staged<Vec<&'static str>>,

    // DO NOT MODIFY FIELDS AFTER THIS POINT IN THE DECLARE AND ITERATE STEPS

    // FEATURES, TRAITS, AND FEATS
    pub race_traits: Vec<Feature>,
    pub class_features: Vec<Feature>,
    pub background_features: Vec<Feature>,
    pub feat_features: Vec<Feature>,

    pub actions: Vec<Move>,
    pub bonus_actions: Vec<Move>,
    pub reactions: Vec<Move>,

    // NOT EDITABLE BY YOU. YES, YOU.

    // FINALIZE MACRO PANICS HERE
    money: [usize; 5],

    inspiration: bool,

    alignment: Alignment,
}

#[derive(Default, Debug)]
pub struct Staged<T>
    where T: Default + Debug + Serialize {
    value: T,
    initializers: HashSet<&'static str>,
    modifiers: HashSet<&'static str>,
    finalizers: HashSet<&'static str>
}

impl<T> Staged<T>
    where T: Serialize + Default + Debug {

    fn new(v: T) -> Self {
        Staged {
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

impl<T> Deref for Staged<T>
    where T: Debug + Default + Serialize {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Staged<T>
    where T: Debug + Default + Serialize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}



unsafe impl Sync for StoredCharacter {}
unsafe impl Send for StoredCharacter {}
unsafe impl Sync for FinalCharacter {}
unsafe impl Send for FinalCharacter {}
