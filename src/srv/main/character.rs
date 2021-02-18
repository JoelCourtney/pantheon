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

    // ARMOR
    pub armor_class: Staged<u32>,

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

    pub acrobatics_proficiency: Staged<ProficiencyType>,
    pub animal_handling_proficiency: Staged<ProficiencyType>,
    pub arcana_proficiency: Staged<ProficiencyType>,
    pub athletics_proficiency: Staged<ProficiencyType>,
    pub deception_proficiency: Staged<ProficiencyType>,
    pub history_proficiency: Staged<ProficiencyType>,
    pub insight_proficiency: Staged<ProficiencyType>,
    pub intimidation_proficiency: Staged<ProficiencyType>,
    pub investigation_proficiency: Staged<ProficiencyType>,
    pub medicine_proficiency: Staged<ProficiencyType>,
    pub nature_proficiency: Staged<ProficiencyType>,
    pub perception_proficiency: Staged<ProficiencyType>,
    pub performance_proficiency: Staged<ProficiencyType>,
    pub persuasion_proficiency: Staged<ProficiencyType>,
    pub religion_proficiency: Staged<ProficiencyType>,
    pub sleight_of_hand_proficiency: Staged<ProficiencyType>,
    pub stealth_proficiency: Staged<ProficiencyType>,
    pub survival_proficiency: Staged<ProficiencyType>,

    // PASSIVES
    pub passive_perception: Staged<i32>,
    pub passive_investigation: Staged<i32>,
    pub passive_insight: Staged<i32>,
    pub passive_notes: Staged<Vec<&'static str>>,

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

    pub strength_save_proficiency: Staged<ProficiencyType>,
    pub dexterity_save_proficiency: Staged<ProficiencyType>,
    pub constitution_save_proficiency: Staged<ProficiencyType>,
    pub intelligence_save_proficiency: Staged<ProficiencyType>,
    pub wisdom_save_proficiency: Staged<ProficiencyType>,
    pub charisma_save_proficiency: Staged<ProficiencyType>,

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
    pub walking_speed: Staged<u32>,
    pub flying_speed: Staged<u32>,
    pub climbing_speed: Staged<u32>,
    pub swimming_speed: Staged<u32>,
    pub burrowing_speed: Staged<u32>,

    // ATTACKS PER ACTION
    pub attacks_per_action: Staged<u32>,

    // MOVES
    pub attack_moves: Staged<Vec<AttackMove>>,
    pub cast_moves: Staged<Vec<CastMove>>,

    // DO NOT MODIFY FIELDS AFTER THIS POINT IN THE DECLARE AND ITERATE STEPS

    // FEATURES, TRAITS, AND FEATS
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

    money: [u32; 5],

    inspiration: bool,

    alignment: Alignment,
}

impl Character {
    // fn get_skill(&self, s: Skill) -> &Staged<i32> {
    //     match s {
    //         Skill::Acrobatics => &self.acrobatics,
    //         Skill::AnimalHandling => &self.animal_handling,
    //         Skill::Arcana => &self.arcana,
    //         Skill::Athletics => &self.athletics,
    //         Skill::Deception => &self.deception,
    //         Skill::History => &self.history,
    //         Skill::Insight => &self.insight,
    //         Skill::Intimidation => &self.intimidation,
    //         Skill::Investigation => &self.investigation,
    //         Skill::Medicine => &self.medicine,
    //         Skill::Nature => &self.nature,
    //         Skill::Perception => &self.perception,
    //         Skill::Performance => &self.performance,
    //         Skill::Persuasion => &self.persuasion,
    //         Skill::Religion => &self.religion,
    //         Skill::SleightOfHand => &self.sleight_of_hand,
    //         Skill::Stealth => &self.stealth,
    //         Skill::Survival => &self.survival,
    //         Skill::Unknown => panic!("cannot get unknown skill")
    //     }
    // }
    //
    // pub(crate) fn get_mut_skill(&mut self, s: Skill) -> &mut Staged<i32> {
    //     match s {
    //         Skill::Acrobatics => &mut self.acrobatics,
    //         Skill::AnimalHandling => &mut self.animal_handling,
    //         Skill::Arcana => &mut self.arcana,
    //         Skill::Athletics => &mut self.athletics,
    //         Skill::Deception => &mut self.deception,
    //         Skill::History => &mut self.history,
    //         Skill::Insight => &mut self.insight,
    //         Skill::Intimidation => &mut self.intimidation,
    //         Skill::Investigation => &mut self.investigation,
    //         Skill::Medicine => &mut self.medicine,
    //         Skill::Nature => &mut self.nature,
    //         Skill::Perception => &mut self.perception,
    //         Skill::Performance => &mut self.performance,
    //         Skill::Persuasion => &mut self.persuasion,
    //         Skill::Religion => &mut self.religion,
    //         Skill::SleightOfHand => &mut self.sleight_of_hand,
    //         Skill::Stealth => &mut self.stealth,
    //         Skill::Survival => &mut self.survival,
    //         Skill::Unknown => panic!("cannot get unknown skill")
    //     }
    // }
    //
    // fn get_skill_proficiency(&self, s: Skill) -> &Staged<ProficiencyType> {
    //     match s {
    //         Skill::Acrobatics => &self.acrobatics_proficiency,
    //         Skill::AnimalHandling => &self.animal_handling_proficiency,
    //         Skill::Arcana => &self.arcana_proficiency,
    //         Skill::Athletics => &self.athletics_proficiency,
    //         Skill::Deception => &self.deception_proficiency,
    //         Skill::History => &self.history_proficiency,
    //         Skill::Insight => &self.insight_proficiency,
    //         Skill::Intimidation => &self.intimidation_proficiency,
    //         Skill::Investigation => &self.investigation_proficiency,
    //         Skill::Medicine => &self.medicine_proficiency,
    //         Skill::Nature => &self.nature_proficiency,
    //         Skill::Perception => &self.perception_proficiency,
    //         Skill::Performance => &self.performance_proficiency,
    //         Skill::Persuasion => &self.persuasion_proficiency,
    //         Skill::Religion => &self.religion_proficiency,
    //         Skill::SleightOfHand => &self.sleight_of_hand_proficiency,
    //         Skill::Stealth => &self.stealth_proficiency,
    //         Skill::Survival => &self.survival_proficiency,
    //         Skill::Unknown => panic!("cannot get unknown skill")
    //     }
    // }

    pub(crate) fn get_mut_skill_proficiency(&mut self, s: Skill) -> &mut Staged<ProficiencyType> {
        match s {
            Skill::Acrobatics => &mut self.acrobatics_proficiency,
            Skill::AnimalHandling => &mut self.animal_handling_proficiency,
            Skill::Arcana => &mut self.arcana_proficiency,
            Skill::Athletics => &mut self.athletics_proficiency,
            Skill::Deception => &mut self.deception_proficiency,
            Skill::History => &mut self.history_proficiency,
            Skill::Insight => &mut self.insight_proficiency,
            Skill::Intimidation => &mut self.intimidation_proficiency,
            Skill::Investigation => &mut self.investigation_proficiency,
            Skill::Medicine => &mut self.medicine_proficiency,
            Skill::Nature => &mut self.nature_proficiency,
            Skill::Perception => &mut self.perception_proficiency,
            Skill::Performance => &mut self.performance_proficiency,
            Skill::Persuasion => &mut self.persuasion_proficiency,
            Skill::Religion => &mut self.religion_proficiency,
            Skill::SleightOfHand => &mut self.sleight_of_hand_proficiency,
            Skill::Stealth => &mut self.stealth_proficiency,
            Skill::Survival => &mut self.survival_proficiency,
            Skill::Unknown => panic!("cannot get unknown skill")
        }
    }
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
