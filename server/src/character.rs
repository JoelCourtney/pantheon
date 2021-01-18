use serde::{Deserialize, Serialize};

use crate::feature::{Feature, Choose};
use crate::misc::*;
use std::fmt::Debug;
use macros::dynamic_choose;
use crate::content::Content;

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

    race: Box<dyn Race>
}

impl StoredCharacter {
    pub fn resolve(&mut self) -> Character {
        let mut char = Character {
            name: self.name.clone(),
            health: self.health,
            temp_health: self.temp_health,

            strength: self.base_strength,
            dexterity: self.base_dexterity,
            constitution: self.base_constitution,
            intelligence: self.base_intelligence,
            wisdom: self.base_wisdom,
            charisma: self.base_charisma,

            alignment: self.alignment,

            ..Default::default()
        };
        self.race.initialize(&mut char);
        self.race.modify(&mut char);
        self.race.finalize(&mut char);
        char.race_traits.extend(self.race.features());
        char
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Character {
    pub name: String,

    // HEALTH
    pub health: usize,
    pub temp_health: usize,
    pub max_health: usize,

    // ABILITIES
    pub strength: usize,
    pub dexterity: usize,
    pub constitution: usize,
    pub intelligence: usize,
    pub wisdom: usize,
    pub charisma: usize,

    pub strength_modifier: i32,
    pub dexterity_modifier: i32,
    pub constitution_modifier: i32,
    pub intelligence_modifier: i32,
    pub wisdom_modifier: i32,
    pub charisma_modifier: i32,

    // INITIATIVE
    pub initiative: i32,

    // SIZE
    pub size: CreatureSize,

    // ALIGNMENT
    pub alignment: Alignment,

    // PROFICIENCIES AND LANGUAGES
    pub skill_proficiencies: Vec<(Skill, ProficiencyType)>,
    pub languages: Vec<Language>,

    // SPEED
    pub walking_speed: usize,
    pub flying_speed: usize,
    pub climbing_speed: usize,
    pub swimming_speed: usize,
    pub burrowing_speed: usize,

    // FEATURES, TRAITS, AND FEATS
    pub race_traits: Vec<Feature>,
    pub class_features: Vec<Feature>,
    pub background_features: Vec<Feature>,
    pub feats: Vec<Vec<Feature>>,

    // NOTES
    pub saving_throw_notes: Vec<&'static str>
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