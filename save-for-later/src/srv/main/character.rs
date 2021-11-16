use serde::{Deserialize, Serialize};

use crate::ui::{Element, Event};
use crate::misc::*;
use std::fmt::Debug;
use proc_macros::FinalizeCharacter;
use crate::content::common::{common_rules, common_race_rules, common_class_rules, common_item_rules, common_background_rules};
use crate::content::traits::{Race, Class, Item, Background};
use std::ops::{Deref, DerefMut};
use std::collections::{HashMap};
use maplit::hashmap;
use crate::moves::*;


impl StoredCharacter {
    pub fn read(path: &str) -> StoredCharacter {
        let json = std::fs::read_to_string(path).expect(&format!("READING FAILED: {}", path));
        serde_json::from_str(&json).expect("DESERIALIZATION FAILED")
    }
    pub fn write(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self).expect("SERIALIZATION FAILED");
        std::fs::write(path, json).expect(&format!("WRITING FAILED: {}", path));
    }

    /// This performs all of the logic of expanding the stored character into a full character.
    ///
    /// 1. Copy/clone some quantities from the stored character directly into the character struct. (like health and alignment)
    /// 2. Repeatedly call resolve on all content attached to the character. The Staged objects,
    ///    combined with the i!, m!, and f! macros (see proc_macros/src/content.rs) will self-organize
    ///    the dependencies.
    pub fn resolve(&mut self) -> Result<FinalCharacter, ()> {
        let mut char = Character {
            health: Staged::new(self.health),
            temp_health: Staged::new(self.temp_health),

            abilities: self.base_abilities.wrap_staged(),

            base_abilities: self.base_abilities.wrap_staged(),

            name: Staged::new(self.name.clone()),
            description: Staged::new(self.description.clone()),

            money: self.money.wrap_staged(),

            inspiration: Staged::new(self.inspiration),

            alignment: Staged::new(self.alignment),

            ..Default::default()
        };

        for _ in 0..self.classes.len() {
            char.class_features.push(Staged::new(vec![]));
        }

        let mut old_count: i64  = -2;
        let mut count: i64 = -1;
        // let mut iterations = 0;
        while count != 0 && old_count != count {
            old_count = count;

            common_rules::resolve(&mut char);
            common_race_rules::resolve(&mut char, &self.race);
            self.race.resolve(&mut char);
            for (i, (class, level)) in self.classes.iter_mut().enumerate() {
                common_class_rules::resolve(&mut char, class, *level, i);
                class.resolve(&mut char, *level, i);
            }
            common_background_rules::resolve(&mut char, &self.background);
            self.background.resolve(&mut char);
            for (item, equipped, attuned) in &mut self.inventory {
                common_item_rules::resolve(&mut char, item, *equipped, *attuned);
                item.resolve(&mut char, *equipped, *attuned);
            }

            count = char.count_unresolved().into();

            // iterations += 1;
        }
        // dbg!(iterations);
        if count != 0 {
            dbg!(&char);
            println!("RESOLUTION DEADLOCK");
            Err(())
        } else {

            Ok(char.finalize())
        }
    }

    pub fn event(&mut self, e: Event) {
        self.race.event(&e);
        for (i, (class, level)) in self.classes.iter_mut().enumerate() {
            class.event(&e, *level, i);
        }
        self.background.event(&e);
        for (item, equipped, attuned) in &mut self.inventory {
            item.event(&e, *equipped, *attuned);
        }
    }
}

impl Default for StoredCharacter {
    fn default() -> Self {
        StoredCharacter {
            name: "".to_string(),
            health: 0,
            temp_health: 0,
            base_abilities: AbilityMap {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            alignment: Alignment::Unknown,
            inspiration: false,
            money: MoneyTypeMap {
                platinum: 0,
                gold: 0,
                electrum: 0,
                silver: 0,
                copper: 0,
            },
            race: crate::content::default_race(),
            background: crate::content::default_background(),
            classes: vec![],
            inventory: vec![],
            description: "".to_string()
        }
    }
}

/// The struct that the StoredCharacter's content is expanded into.
///
/// I've created a derive proc macro `FinalizeCharacter` that produces a
/// FinalCharacter struct, which has all the same fields, except the Staged<>
/// values are unwrapped, and also produces a consumer method that converts Character
/// into FinalCharacter. FinalCharacter is what is serialized and sent to the frontend.
#[derive(Debug, Default, FinalizeCharacter)]
pub struct Character {
    pub name: Staged<String>,
    pub total_level: Staged<u32>,
    pub race_name: Staged<String>,
    pub class_names: Staged<Vec<String>>,
    pub class_levels: Staged<Vec<u32>>,
    pub background_name: Staged<String>,

    // PROFICIENCY BONUS AND INITIATIVE
    pub proficiency_bonus: Staged<u32>,
    pub initiative: Staged<i32>,

    // HEALTH
    pub health: Staged<u32>,
    pub temp_health: Staged<u32>,
    pub max_health: Staged<u32>,

    // ARMOR
    pub armor_class: Staged<u32>,

    // ABILITIES
    pub abilities: AbilityMap<Staged<u32>>,
    pub ability_modifiers: AbilityMap<Staged<i32>>,

    // SKILLS
    pub skills: SkillMap<Staged<i32>>,
    pub skill_vantages: SkillMap<Staged<Vantage>>,
    pub skill_proficiencies: SkillMap<Staged<ProficiencyType>>,

    // PASSIVES
    pub passives: PassiveSkillMap<Staged<i32>>,
    pub passive_notes: Staged<Vec<&'static str>>,

    // SAVING THROWS
    pub saves: AbilityMap<Staged<i32>>,
    pub save_vantages: AbilityMap<Staged<Vantage>>,
    pub save_proficiencies: AbilityMap<Staged<ProficiencyType>>,
    pub saving_throw_notes: Staged<Vec<&'static str>>,

    // SIZE
    pub size: Staged<CreatureSize>,

    // PROFICIENCIES AND LANGUAGES
    pub tool_proficiencies: Staged<Vec<(&'static str, ProficiencyType)>>,
    pub weapon_proficiencies: Staged<Vec<&'static str>>,
    pub armor_proficiencies: Staged<Vec<&'static str>>,
    pub languages: Staged<Vec<Language>>,

    // DEFENSES, CONDITIONS
    pub defenses: Staged<Vec<&'static str>>,
    pub conditions: Staged<Vec<&'static str>>,

    // SPEED
    pub speeds: MovementTypeMap<Staged<u32>>,

    // ATTACKS PER ACTION
    pub attacks_per_action: Staged<u32>,

    // MOVES
    pub moves: Staged<Vec<Move>>,

    // FEATURES, TRAITS, AND FEATS
    pub race_choices: Staged<Vec<&'static str>>,
    pub class_choices: Staged<Vec<&'static str>>,
    pub background_choices: Staged<Vec<&'static str>>,

    pub race_traits: Staged<Vec<Element<'static>>>,
    pub class_features: Vec<Staged<Vec<Element<'static>>>>,
    pub background_features: Staged<Vec<Element<'static>>>,
    pub feats: Staged<Vec<Element<'static>>>,

    // NOT EDITABLE BY YOU. YES, YOU.

    pub left_hand: Staged<Option<&'static str>>,
    pub right_hand: Staged<Option<&'static str>>,
    pub both_hands: Staged<Option<&'static str>>,
    pub ammunition: Staged<Option<&'static str>>,
    pub armor: Staged<Option<&'static str>>,

    pub hold_choices: Staged<Vec<&'static str>>,
    pub armor_choices: Staged<Vec<&'static str>>,
    pub ammunition_choices: Staged<Vec<&'static str>>,

    pub money: MoneyTypeMap<Staged<u32>>,

    pub inspiration: Staged<bool>,

    pub alignment: Staged<Alignment>,

    pub description: Staged<String>,

    pub base_abilities: AbilityMap<Staged<u32>>
}

pub trait Resolveable {
    type Inner;

    fn count_unresolved(&self) -> u32;
    fn unwrap(self) -> Self::Inner;
}

#[derive(Default, Debug, Serialize)]
pub struct Staged<T>
    where T: Default + Debug + Serialize {
    value: T,
    initializers: HashMap<u64, bool>,
    modifiers: HashMap<u64, bool>,
    finalizers: HashMap<u64, bool>,
}

impl<T> Staged<T>
    where T: Serialize + Default + Debug {

    pub fn new(v: T) -> Self {
        Staged {
            value: v,
            initializers: hashmap! {},
            modifiers: hashmap! {},
            finalizers: hashmap! {}
        }
    }


    fn initialized(&self) -> bool {
        self.initializers.values().all(|b| *b)
    }
    fn modified(&self) -> bool {
        self.initialized() && self.modifiers.values().all(|b| *b)
    }
    pub fn finalized(&self) -> bool {
        self.modified() && self.finalizers.values().all(|b| *b)
    }

    pub fn request_initialize(&mut self, who: u64) -> bool {
        match self.initializers.get(&who) {
            Some(b) if !*b => true,
            None => {
                self.initializers.insert(who, false);
                false
            }
            _ => false
        }
    }
    pub fn request_modify(&mut self, who: u64) -> bool {
        match self.modifiers.get(&who) {
            Some(b) if !*b => self.initialized(),
            None => {
                self.modifiers.insert(who, false);
                false
            }
            _ => false
        }
    }
    pub fn request_finalize(&mut self, who: u64) -> bool {
        match self.finalizers.get(&who) {
            Some(b) if !*b => self.initialized() && self.modified(),
            None => {
                self.finalizers.insert(who, false);
                false
            }
            _ => false
        }
    }

    pub fn confirm_initialize(&mut self, who: u64) {
        match self.initializers.get_mut(&who) {
            Some(b) if !*b => *b = true,
            _ => panic!("nope")
        }
    }
    pub fn confirm_modify(&mut self, who: u64) {
        match self.modifiers.get_mut(&who) {
            Some(b) if !*b => *b = true,
            _ => panic!("nope")
        }
    }
    pub fn confirm_finalize(&mut self, who: u64) {
        match self.finalizers.get_mut(&who) {
            Some(b) if !*b => *b = true,
            _ => panic!("nope")
        }
    }
}

impl<T> Resolveable for Staged<T>
    where T: Serialize + Default + Debug {
    type Inner = T;

    fn count_unresolved(&self) -> u32 {
        self.initializers.values().fold(0, |acc, b| acc + !*b as u32)
            + self.modifiers.values().fold(0, |acc, b| acc + !*b as u32)
            + self.finalizers.values().fold(0, |acc, b| acc + !*b as u32)
    }
    fn unwrap(self) -> T {
    self.value
}
}

impl<T> Staged<T>
    where T: Serialize + Default + Debug + Clone {
    pub fn r#final(&self) -> Result <T, () > {
        if self.finalized() {
            Ok(self.value.clone())
        } else {
            Err(())
        }
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

impl<T> Resolveable for Vec<Staged<T>>
    where T: Debug + Default + Serialize {
    type Inner = Vec<T>;

    fn count_unresolved(&self) -> u32 {
        let mut acc = 0;
        for stage in self {
            acc += stage.count_unresolved();
        }
        acc
    }

    fn unwrap(self) -> Vec<T> {
        self.into_iter().map(|stage| stage.unwrap()).collect()
    }
}

unsafe impl Sync for StoredCharacter {}
unsafe impl Send for StoredCharacter {}
unsafe impl Sync for FinalCharacter {}
unsafe impl Send for FinalCharacter {}
