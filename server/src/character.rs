use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
}

impl StoredCharacter {
    pub fn resolve(self) -> Character {
        let char = Character {
            name: self.name,
            health: self.health,
            temp_health: self.temp_health,

            strength: self.base_strength,
            dexterity: self.base_dexterity,
            constitution: self.base_constitution,
            intelligence: self.base_intelligence,
            wisdom: self.base_wisdom,
            charisma: self.base_charisma,

            ..Default::default()
        };
        char
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Character {
    name: String,

    health: u64,
    temp_health: u64,
    max_health: u64,

    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,

    strength_modifier: i8,
    dexterity_modifier: i8,
    constitution_modifier: i8,
    intelligence_modifier: i8,
    wisdom_modifier: i8,
    charisma_modifier: i8,
}