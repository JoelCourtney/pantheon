use serde::{Deserialize, Serialize};

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
    Other(String)
}

impl Default for Language {
    fn default() -> Self {
        Language::Unspecified
    }
}