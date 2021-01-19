use serde::{Serialize, Deserialize};
use crate::character::Character;

pub struct CommonRules;

impl CommonRules {
    const NAME: &'static str = "Common Rules";

    pub fn declare(c: &mut Character) {
        c.strength_modifier.declare_initializer(Self::NAME);
        c.dexterity_modifier.declare_initializer(Self::NAME);
        c.constitution_modifier.declare_initializer(Self::NAME);
        c.intelligence_modifier.declare_initializer(Self::NAME);
        c.wisdom_modifier.declare_initializer(Self::NAME);
        c.charisma_modifier.declare_initializer(Self::NAME);
    }
    pub fn modify(c: &mut Character) {
        if c.strength.finalized() && c.strength_modifier.initialize(Self::NAME) {
            *c.strength_modifier = (*c.strength as i32 - 10) / 2;
        }
        if c.dexterity.finalized() && c.dexterity_modifier.initialize(Self::NAME) {
            *c.dexterity_modifier = (*c.dexterity as i32 - 10) / 2;
        }
        if c.constitution.finalized() && c.constitution_modifier.initialize(Self::NAME) {
            *c.constitution_modifier = (*c.constitution as i32 - 10) / 2;
        }
        if c.intelligence.finalized() && c.intelligence_modifier.initialize(Self::NAME) {
            *c.intelligence_modifier = (*c.intelligence as i32 - 10) / 2;
        }
        if c.wisdom.finalized() && c.wisdom_modifier.initialize(Self::NAME) {
            *c.wisdom_modifier = (*c.wisdom as i32 - 10) / 2;
        }
        if c.charisma.finalized() && c.charisma_modifier.initialize(Self::NAME) {
            *c.charisma_modifier = (*c.charisma as i32 - 10) / 2;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CommonClassContent<const HD: usize> {
    pub level: usize,
    first_class: bool
}

impl<const HD: usize> CommonClassContent<HD> {
    const NAME: &'static str = "Common Class Content";

    pub fn declare(&self, c: &mut Character) {
        c.max_health.declare_finalizer(Self::NAME);
    }
    pub fn modify(&self, c: &mut Character) {
        if c.constitution_modifier.finalized() && c.max_health.finalize(Self::NAME) {
            let mut res: i32 = 0;
            if self.first_class && self.level >= 1 {
                res += (HD / 2 - 1) as i32;
            }
            res += ((HD / 2 + 1) as i32 + *c.constitution_modifier) * self.level as i32;
            *c.max_health += res as usize;
        }
    }
}
