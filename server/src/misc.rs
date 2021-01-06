use serde::{Deserialize, Serialize};
use crate::feature::{Choice, Choose};
use macros::Choose;

#[derive(Debug, Deserialize, Serialize, Choose)]
pub enum CreatureSize {
    Unknown,
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

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Choose)]
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
    Unknown,
    ItsComplicated
}

#[derive(Debug, Deserialize, Serialize, Choose)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    Unknown
}

#[derive(Debug, Deserialize, Serialize, Choose)]
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
    Survival,
    Unknown
}

#[derive(Debug, Deserialize, Serialize, Choose)]
pub enum SavingThrow {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    Death,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Choose)]
pub enum ProficiencyType {
    NONE,
    HALF,
    SINGLE,
    DOUBLE,
    Unknown
}

#[derive(Choose, Debug, Deserialize, Serialize, Clone)]
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
    Unknown,
}
