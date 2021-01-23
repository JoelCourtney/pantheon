use serde::{Deserialize, Serialize};
use crate::feature::Choose;
use macros::choose;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct RolledAmount {
    pub dice: HashMap<u32, i32>,
    pub constant: i32
}

#[derive(Debug, Serialize, Copy, Clone)]
pub enum Range {
    Fixed(u32),
    Extra(u32, u32)
}

impl Default for Range {
    fn default() -> Self {
        Range::Fixed(5)
    }
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

#[derive(Debug, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Equipable {
    No,
    Yes,
    Always,
    Armor,
    Holdable(Holdable)
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum Equipped {
    No,
    Yes,
    HeldVersatile(Hand)
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Hand {
    Left,
    Right,
    Both
}

#[derive(Debug, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Holdable {
    One,
    Two,
    Versatile,
    Ammunition
}

#[derive(Debug, Serialize, Copy, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Vantage {
    Advantage,
    None,
    Disadvantage,
    NoneLocked
}

impl Vantage {
    // WILL UNCOMMENT LATER
    pub(crate) fn upgrade(&mut self) {
        use Vantage::*;

        match self {
            None => {
                *self = Advantage;
            }
            Disadvantage => {
                *self = NoneLocked;
            }
            _ => {}
        }
    }

    // fn downgrade(&mut self) {
    //     use Vantage::*;
    //
    //     match self {
    //         None => {
    //             *self = Disadvantage;
    //         }
    //         Advantage => {
    //             *self = NoneLocked;
    //         }
    //         _ => {}
    //     }
    // }
}

impl Default for Vantage {
    fn default() -> Self { Vantage::None }
}