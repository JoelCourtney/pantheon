use serde::{Deserialize, Serialize};

use crate::feature::Feature;
use crate::misc::*;
use std::fmt::Debug;
use macros::FinalizeCharacter;
use crate::content::common::common_rules;
use crate::content::traits::{Race, Class, Item};
use std::ops::{Deref, DerefMut};
use std::collections::HashSet;
use maplit::hashset;
use crate::moves::Move;

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    pub(crate) name: String,

    pub(crate) health: u32,
    pub(crate) temp_health: u32,

    base_strength: u32,
    base_dexterity: u32,
    base_constitution: u32,
    base_intelligence: u32,
    base_wisdom: u32,
    base_charisma: u32,

    alignment: Alignment,

    inspiration: bool,

    money: [u32; 5],

    race: Box<dyn Race>,
    classes: Vec<(Box<dyn Class>, u32)>,

    inventory: Vec<(Box<dyn Item>, Equipped, bool)>
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
        for (i, (class, level)) in self.classes.iter().enumerate() {
            let first = i == 0;
            class.declare(&mut char, *level, first);
        }
        for (item, equipped, attuned) in &self.inventory {
            item.declare(&mut char, *equipped, *attuned);
        }

        let mut old_count: i64  = -2;
        let mut count: i64 = -1;
        // let mut iterations = 0;
        while count != 0 && old_count != count {
            old_count = count;

            common_rules::iterate(&mut char);
            self.race.iterate(&mut char);
            for (i, (class, level)) in self.classes.iter().enumerate() {
                let first = i == 0;
                class.iterate(&mut char, *level, first);
            }
            for (item, equipped, attuned) in &self.inventory {
                item.iterate(&mut char, *equipped, *attuned);
            }

            count = char.count_unresolved().into();

            // iterations += 1;
        }
        // dbg!(iterations);
        if count != 0 {
            dbg!(&char);
            println!("ITERATOR DEADLOCK");
            Err(())
        } else {
            common_rules::last(&mut char);
            self.race.last(&mut char);
            for (i, (class, level)) in self.classes.iter_mut().enumerate() {
                let first = i == 0;
                class.last(&mut char, *level, first);
            }
            Ok(char.finalize())
        }
    }
}

#[derive(Debug, Default, FinalizeCharacter)]
pub struct Character {
    pub name: Staged<String>,
    pub total_level: Staged<u32>,
    pub race_name: Staged<String>,
    pub class_names: Staged<Vec<String>>,

    // PROFICIENCY BONUS AND INITIATIVE
    pub proficiency_bonus: Staged<u32>,
    pub initiative: Staged<i32>,

    // HEALTH
    pub health: Staged<u32>,
    pub temp_health: Staged<u32>,
    pub max_health: Staged<u32>,

    // ABILITIES
    pub strength: Staged<u32>,
    pub dexterity: Staged<u32>,
    pub constitution: Staged<u32>,
    pub intelligence: Staged<u32>,
    pub wisdom: Staged<u32>,
    pub charisma: Staged<u32>,

    pub strength_modifier: Staged<i32>,
    pub dexterity_modifier: Staged<i32>,
    pub constitution_modifier: Staged<i32>,
    pub intelligence_modifier: Staged<i32>,
    pub wisdom_modifier: Staged<i32>,
    pub charisma_modifier: Staged<i32>,

    // SKILLS
    pub acrobatics: Staged<i32>,
    pub animal_handling: Staged<i32>,
    pub arcana: Staged<i32>,
    pub athletics: Staged<i32>,
    pub deception: Staged<i32>,
    pub history: Staged<i32>,
    pub insight: Staged<i32>,
    pub intimidation: Staged<i32>,
    pub investigation: Staged<i32>,
    pub medicine: Staged<i32>,
    pub nature: Staged<i32>,
    pub perception: Staged<i32>,
    pub performance: Staged<i32>,
    pub persuasion: Staged<i32>,
    pub religion: Staged<i32>,
    pub sleight_of_hand: Staged<i32>,
    pub stealth: Staged<i32>,
    pub survival: Staged<i32>,

    pub acrobatics_vantage: Staged<Vantage>,
    pub animal_handling_vantage: Staged<Vantage>,
    pub arcana_vantage: Staged<Vantage>,
    pub athletics_vantage: Staged<Vantage>,
    pub deception_vantage: Staged<Vantage>,
    pub history_vantage: Staged<Vantage>,
    pub insight_vantage: Staged<Vantage>,
    pub intimidation_vantage: Staged<Vantage>,
    pub investigation_vantage: Staged<Vantage>,
    pub medicine_vantage: Staged<Vantage>,
    pub nature_vantage: Staged<Vantage>,
    pub perception_vantage: Staged<Vantage>,
    pub performance_vantage: Staged<Vantage>,
    pub persuasion_vantage: Staged<Vantage>,
    pub religion_vantage: Staged<Vantage>,
    pub sleight_of_hand_vantage: Staged<Vantage>,
    pub stealth_vantage: Staged<Vantage>,
    pub survival_vantage: Staged<Vantage>,

    // SAVING THROWS
    pub strength_save: Staged<i32>,
    pub dexterity_save: Staged<i32>,
    pub constitution_save: Staged<i32>,
    pub intelligence_save: Staged<i32>,
    pub wisdom_save: Staged<i32>,
    pub charisma_save: Staged<i32>,

    pub strength_save_vantage: Staged<Vantage>,
    pub dexterity_save_vantage: Staged<Vantage>,
    pub constitution_save_vantage: Staged<Vantage>,
    pub intelligence_save_vantage: Staged<Vantage>,
    pub wisdom_save_vantage: Staged<Vantage>,
    pub charisma_save_vantage: Staged<Vantage>,

    pub saving_throw_notes: Staged<Vec<&'static str>>,

    // SIZE
    pub size: Staged<CreatureSize>,

    // PROFICIENCIES AND LANGUAGES
    pub skill_proficiencies: Staged<Vec<(Skill, ProficiencyType)>>,
    pub tool_proficiencies: Staged<Vec<(&'static str, ProficiencyType)>>,
    pub languages: Staged<Vec<Language>>,

    // SPEED
    pub walking_speed: Staged<u32>,
    pub flying_speed: Staged<u32>,
    pub climbing_speed: Staged<u32>,
    pub swimming_speed: Staged<u32>,
    pub burrowing_speed: Staged<u32>,

    // ATTACKS PER ACTION
    pub attacks_per_action: Staged<u32>,

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
    money: [u32; 5],

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
