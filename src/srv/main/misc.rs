use serde::{Deserialize, Serialize, Serializer};
use crate::feature::Choose;
use std::collections::HashMap;
use maplit::hashmap;
use std::fmt::{Display, Formatter};
use crate::choose;

/// Represents an amount of damage.
///
/// Random dice rolls are stored in `dice`. NdS is stored as `S => N`. When modifying the dice
/// component to add or remove some S sided dice, you should in most cases check if S is already
/// in the map. Negative N are allowed.
///
/// Constant is the predetermined component. E.g. in "2d4 + 3", 3 is the constant.
#[derive(Debug)]
pub struct Damage {
    pub dice: HashMap<u32, i32>,
    pub constant: i32,
    pub ty: DamageType
}

impl Damage {
    /// Creates a Damage struct for a single die (1dS) of a given damage type.
    pub fn from_die(s: u32, ty: DamageType) -> Damage {
        Damage {
            dice: hashmap! {
                s => 1
            },
            constant: 0,
            ty
        }
    }

    /// Creates a Damage struct for multiple dice of the same kind (NdS) of a given damage type.
    #[allow(dead_code)]
    fn from_dice(n: i32, s: u32, ty: DamageType) -> Damage {
        Damage {
            dice: hashmap! {
                s => n
            },
            constant: 0,
            ty
        }
    }
}

impl Serialize for Damage {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut res = String::new();
        for (s, n) in &self.dice {
            res += &*format!("+{}d{}", n, s);
        }
        if self.constant != 0 {
            res += &*format!("+{}", self.constant);
        }
        res += &*format!(" {}", self.ty);
        serializer.serialize_str(&res[1..])
    }
}

/// Please don't make me let you homebrew this.
#[derive(Debug, Serialize)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
    Other(&'static str)
}

impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageType::Other(s) => write!(f, "{}", s),
            _ => write!(f, "{:?}", self)
        }
    }
}

/// Represents a range.
///
/// Myself is used in place of `Self` because that is a reserved keyword. Fixed is for ranges with
/// a hard limit, and Tiered is for two-part ranges where the farther range has disadvantage.
#[derive(Debug, Serialize, Copy, Clone)]
pub enum Range {
    Myself,
    Touch,
    Fixed(u32),
    Tiered(u32, u32)
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

/// Whether the item can be equipped, and how.
///
/// - **No**: Not equippable, or does not do anything when equipped.
/// - **Yes**: Equippable by choice, and is not armor or holdable.
/// - **Always**: Is always automatically equipped if in the character's inventory.
/// - **Armor**: Is armor.
/// - **Holdable**: Must be held in order to have any effect. E.G. weapons, shields, ammunition
#[derive(Debug, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Equipable {
    No,
    Yes,
    Always,
    Armor,
    Holdable(Holdable)
}

/// What type of holdable item it is.
///
/// - **One**: always held in one hand
/// - **Two**: always held in two hands
/// - **Versatile**: can be held in one or two hands
/// - **Ammunition**: only usable when an ammunition-using weapon is currently held.
#[derive(Debug, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Holdable {
    One,
    Two,
    Versatile,
    Ammunition
}

/// Current Equipped status of an item.
///
/// - **No**: not equipped
/// - **Yes**: equipped, when item is not versatile
/// - **HeldVersatile**: equipped, when it is versatile
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum Equipped {
    No,
    Yes,
    Held(Hand)
}

/// Which hand a versatile item is held in.
#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Hand {
    Left,
    Right,
    Both
}

choose! {
    pub enum Rarity {
        Common,
        Uncommon,
        Rare,
        #[choose(pretty = "Very Rare")]
        VeryRare,
        Legendary,
        Unknown
    }

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

    pub enum Alignment {

        #[choose(pretty = "Lawful Good")]
        LawfulGood,

        #[choose(pretty = "Lawful Neutral")]
        LawfulNeutral,

        #[choose(pretty = "Lawful Evil")]
        LawfulEvil,

        #[choose(pretty = "Neutral Good")]
        NeutralGood,

        #[choose(pretty = "True Neutral")]
        TrueNeutral,

        #[choose(pretty = "Neutral Evil")]
        NeutralEvil,

        #[choose(pretty = "Chaotic Good")]
        ChaoticGood,

        #[choose(pretty = "Chaotic Neutral")]
        ChaoticNeutral,

        #[choose(pretty = "Chaotic Evil")]
        ChaoticEvil,

        Unknown,

        #[choose(pretty = "It's complicated.")]
        ItsComplicated
    }

    pub enum Ability {
        Strength,
        Dexterity,
        Constitution,
        Intelligence,
        Wisdom,
        Charisma,
        Unknown
    }

    pub enum Skill {
        Acrobatics,
        #[choose(pretty = "Animal Handling")]
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
        #[choose(pretty = "Sleight of Hand")]
        SleightOfHand,
        Stealth,
        Survival,
        Unknown
    }
}

impl Skill {
    pub fn get_associated_ability(&self) -> Ability {
        use Ability::*;
        match self {
            Skill::Acrobatics => Dexterity,
            Skill::AnimalHandling => Wisdom,
            Skill::Arcana => Intelligence,
            Skill::Athletics => Strength,
            Skill::Deception => Charisma,
            Skill::History => Intelligence,
            Skill::Insight => Wisdom,
            Skill::Intimidation => Charisma,
            Skill::Investigation => Intelligence,
            Skill::Medicine => Wisdom,
            Skill::Nature => Intelligence,
            Skill::Perception => Wisdom,
            Skill::Performance => Charisma,
            Skill::Persuasion => Charisma,
            Skill::Religion => Intelligence,
            Skill::SleightOfHand => Dexterity,
            Skill::Stealth => Dexterity,
            Skill::Survival => Wisdom,
            Skill::Unknown => Unknown
        }
    }
}

choose! {
    pub enum PassiveSkill {
        Perception,
        Investigation,
        Insight,
        Unknown
    }
}

impl PassiveSkill {
    pub fn into_skill(self) -> Skill {
        use Skill::*;
        match self {
            PassiveSkill::Perception => Perception,
            PassiveSkill::Investigation => Investigation,
            PassiveSkill::Insight => Insight,
            PassiveSkill::Unknown => Unknown
        }
    }
}

choose! {
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
}

#[derive(Debug, Serialize, Copy, Clone)]
pub enum ProficiencyType {
    None,
    Half,
    Single,
    Double
}

impl Default for ProficiencyType {
    fn default() -> Self {
        Self::None
    }
}

choose! {
    pub enum Language {
        Abyssal,
        Aquan,
        Auran,
        Celestial,
        Common,
        #[choose(pretty = "Deep Speech")]
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
}

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Vantage {
    Advantage,
    None,
    Disadvantage,
    NoneLocked
}

impl Vantage {
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

    #[allow(dead_code)]
    pub(crate) fn downgrade(&mut self) {
        use Vantage::*;

        match self {
            None => {
                *self = Disadvantage;
            }
            Advantage => {
                *self = NoneLocked;
            }
            _ => {}
        }
    }
}

impl Default for Vantage {
    fn default() -> Self { Vantage::None }
}

choose! {
    pub enum MovementType {
        Walk,
        Fly,
        Climb,
        Burrow,
        Swim,
        Unknown
    }

    pub enum MoneyType {
        Platinum,
        Gold,
        Electrum,
        Silver,
        Copper,
        Unknown
    }
}