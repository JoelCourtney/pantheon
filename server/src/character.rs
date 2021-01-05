use serde::{Deserialize, Serialize};

use crate::modify::*;
use crate::feature::{Choice, Feature};

use crate::content::official::players_handbook::races::human::Human;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct StoredCharacter {
    name: String,

    health: u64,
    temp_health: u64,

    base_strength: u8,
    base_dexterity: u8,
    base_constitution: u8,
    base_intelligence: u8,
    base_wisdom: u8,
    base_charisma: u8,

    alignment: Alignment,

    #[serde(skip)]
    human: Human
}

impl StoredCharacter {
    pub fn copy_to_full_character(self) -> Character {
        let mut char = Character {
            name: self.name,
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
        self.human.initialize(&mut char);
        self.human.modify(&mut char);
        self.human.finalize(&mut char);
        char.human = self.human;

        char
    }
}

#[derive(Debug, Default)]
pub(crate) struct Character {
    pub human: Human,
    pub name: String,

    // HEALTH
    pub health: u64,
    pub temp_health: u64,
    pub max_health: u64,

    // ABILITIES
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,

    pub strength_modifier: i8,
    pub dexterity_modifier: i8,
    pub constitution_modifier: i8,
    pub intelligence_modifier: i8,
    pub wisdom_modifier: i8,
    pub charisma_modifier: i8,

    // SIZE
    pub size: CreatureSize,

    // ALIGNMENT
    pub alignment: Alignment,

    // PROFICIENCIES
    pub proficiencies: Vec<Proficiency>,

    // SPEED
    pub walking_speed: u8,
    pub flying_speed: u8,
    pub climbing_speed: u8,
    pub swimming_speed: u8,
    pub burrowing_speed: u8,

    // FEATURES AND TRAITS
    pub traits: Vec<Feature>,
    pub features: Vec<Feature>,
}

impl Character {
    pub(crate) fn resolve(&mut self) {
        self.traits.append(&mut self.human.traits());
        let mut traits = self.human.traits();
        match traits[1].choice {
            Choice::Language(ref mut l) => unsafe {
                **l = Language::Common;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum CreatureSize {
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

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Alignment {
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
pub(crate) enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Skill {
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
pub(crate) enum SavingThrow {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    Death
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum ProficiencyBoost {
    NONE,
    HALF,
    SINGLE,
    DOUBLE
}

// TODO("update String types later")
#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Proficiency {
    Skill(Skill, ProficiencyBoost),
    Tool(String, ProficiencyBoost),
    Language(Language),
    SavingThrow(SavingThrow),
    Armor(String),
    Weapon(String)
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Language {
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