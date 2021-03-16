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
use crate::moves::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    pub(crate) name: String,

    pub(crate) health: u32,
    pub(crate) temp_health: u32,

    base_abilities: AbilityMap<u32>,

    alignment: Alignment,

    inspiration: bool,

    money: MoneyTypeMap<u32>,

    pub(crate) race: Box<dyn Race>,
    classes: Vec<(Box<dyn Class>, u32)>,

    inventory: Vec<(Box<dyn Item>, Equipped, bool)>,

    pub(crate) description: String
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
            health: Staged::new(self.health),
            temp_health: Staged::new(self.temp_health),

            abilities: AbilityMap {
                strength: Staged::new(self.base_abilities.strength),
                dexterity: Staged::new(self.base_abilities.dexterity),
                constitution: Staged::new(self.base_abilities.constitution),
                intelligence: Staged::new(self.base_abilities.intelligence),
                wisdom: Staged::new(self.base_abilities.wisdom),
                charisma: Staged::new(self.base_abilities.charisma),
            },

            name: self.name.clone(),
            description: self.description.clone(),

            money: self.money.clone(),
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
            for (item, equipped, attuned) in &mut self.inventory {
                item.last(&mut char, *equipped, *attuned);
                match item.equipable() {
                    Equipable::Armor => {
                        match equipped {
                            Equipped::Yes => char.armor = Some(item.name()),
                            Equipped::No => char.armor_choices.push(item.name()),
                            Equipped::Held(_) => panic!("armor isn't holdable")
                        }
                    }
                    Equipable::Holdable(hold) => {
                        match hold {
                            Holdable::Ammunition => {
                                match equipped {
                                    Equipped::Yes => char.ammunition = Some(item.name()),
                                    Equipped::No => char.ammunition_choices.push(item.name()),
                                    Equipped::Held(_) => panic!("ammunition isn't holdable")
                                }
                            }
                            _ => {
                                match equipped {
                                    Equipped::Held(hand) => {
                                        match hand {
                                            Hand::Left => char.left_hand = Some(item.name()),
                                            Hand::Right => char.right_hand = Some(item.name()),
                                            Hand::Both => char.both_hands = Some(item.name())
                                        }
                                    }
                                    Equipped::No => {
                                        char.hold_choices.push(item.name());
                                    }
                                    Equipped::Yes => panic!("holdables aren't generically equippable")
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(char.finalize())
        }
    }
}

#[derive(Debug, Default, FinalizeCharacter)]
pub struct Character {
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
    pub attack_moves: Staged<Vec<AttackMove>>,
    pub cast_moves: Staged<Vec<CastMove>>,

    // DO NOT MODIFY FIELDS AFTER THIS POINT IN THE DECLARE AND ITERATE STEPS

    name: String,

    // FEATURES, TRAITS, AND FEATS
    pub race_choices: Vec<&'static str>,
    pub class_choices: Vec<&'static str>,
    pub background_choices: Vec<&'static str>,

    pub race_traits: Vec<Feature>,
    pub class_features: Vec<Feature>,
    pub background_features: Vec<Feature>,
    pub feat_features: Vec<Feature>,

    pub misc_moves: Vec<MiscMove>,

    // NOT EDITABLE BY YOU. YES, YOU.

    left_hand: Option<&'static str>,
    right_hand: Option<&'static str>,
    both_hands: Option<&'static str>,
    ammunition: Option<&'static str>,
    armor: Option<&'static str>,

    hold_choices: Vec<&'static str>,
    armor_choices: Vec<&'static str>,
    ammunition_choices: Vec<&'static str>,

    money: MoneyTypeMap<u32>,

    inspiration: bool,

    alignment: Alignment,

    description: String
}

#[derive(Default, Debug, Serialize)]
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

    pub fn ready(&self) -> bool {
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
