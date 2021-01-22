use serde::{Deserialize, Serialize};
use crate::feature::Choose;
use macros::choose;

#[derive(Debug, Serialize)]
pub enum Range {
    Fixed(usize),
    Extra(usize, usize)
}

#[derive(Debug, Serialize)]
pub enum CastingTime {
    Action,
    BonusAction,
    Reaction,
    Ritual(&'static str)
}

#[derive(Debug, Serialize)]
pub struct CastingComponents {
    verbal: bool,
    somatic: bool,
    material: Option<&'static str>
}

#[derive(Debug, Serialize)]
pub enum Equipable {
    No,
    Yes,
    Armor,
    Holdable(Holdable)
}

#[derive(Debug, Serialize)]
pub enum Holdable {
    One,
    Two,
    Versatile,
    Ammunition
}

#[derive(Debug, Serialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary
}

#[choose]
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

#[choose]
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

#[choose]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    Unknown
}

#[choose]
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

#[choose]
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

#[choose]
pub enum ProficiencyType {
    None,
    Half,
    Single,
    Double,
    Unknown
}

#[choose]
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

