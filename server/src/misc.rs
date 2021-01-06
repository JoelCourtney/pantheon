use serde::{Deserialize, Serialize};
use crate::feature::{Choice, Choose};

#[derive(Debug, Deserialize, Serialize)]
pub enum CreatureSize {
    Unspecified,
    Fine,
    Diminutive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal
}

impl Default for CreatureSize {
    fn default() -> Self { CreatureSize::Unspecified }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
    Unspecified,
    ItsComplicated
}

impl Default for Alignment {
    fn default() -> Self { Alignment::Unspecified }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SavingThrow {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    Death
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProficiencyType {
    NONE,
    HALF,
    SINGLE,
    DOUBLE
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Language {
    Abyssal,
    Aquan,
    Auran,
    Celestial,
    Common,
    DeepSpeech,
    Draconic,
    Druidic,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Gnoll,
    Halfling,
    Ignan,
    Infernal,
    Orc,
    Primordial,
    Sylvan,
    Terran,
    Undercommon,
    Unspecified,
}

impl Choose for Language {
    fn choice<'a>(loc : &'a mut Self) -> Box<dyn Choice + 'a> {
        Box::new( LanguageChoice { loc } )
    }
}

#[derive(Debug)]
pub struct LanguageChoice<'a> {
    loc: &'a mut Language
}

impl Choice for LanguageChoice<'_> {
    fn choices(&self) -> Vec<&str> {
        vec!["Common", "Terran"]
    }
    fn choose(&mut self, choice: &str) {
        *self.loc = match choice {
            "Common" => Language::Common,
            "Terran" => Language::Terran,
            _ => Language::Unspecified
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Unspecified
    }
}