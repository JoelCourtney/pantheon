use serde::{Serialize, Deserialize};
use crate::character::Character;
use crate::content::traits::Class;

pub(crate) mod common_rules {
    use crate::character::Character;

    const NAME: &'static str = "Common Rules";

    pub fn declare(c: &mut Character) {
        c.strength_modifier.declare_initializer(NAME);
        c.dexterity_modifier.declare_initializer(NAME);
        c.constitution_modifier.declare_initializer(NAME);
        c.intelligence_modifier.declare_initializer(NAME);
        c.wisdom_modifier.declare_initializer(NAME);
        c.charisma_modifier.declare_initializer(NAME);

        c.attacks_per_action.declare_initializer(NAME);
    }
    pub fn iterate(c: &mut Character) {
        if c.strength.finalized() && c.strength_modifier.initialize(NAME) {
            *c.strength_modifier = (*c.strength as i32 - 10) / 2;
        }
        if c.dexterity.finalized() && c.dexterity_modifier.initialize(NAME) {
            *c.dexterity_modifier = (*c.dexterity as i32 - 10) / 2;
        }
        if c.constitution.finalized() && c.constitution_modifier.initialize(NAME) {
            *c.constitution_modifier = (*c.constitution as i32 - 10) / 2;
        }
        if c.intelligence.finalized() && c.intelligence_modifier.initialize(NAME) {
            *c.intelligence_modifier = (*c.intelligence as i32 - 10) / 2;
        }
        if c.wisdom.finalized() && c.wisdom_modifier.initialize(NAME) {
            *c.wisdom_modifier = (*c.wisdom as i32 - 10) / 2;
        }
        if c.charisma.finalized() && c.charisma_modifier.initialize(NAME) {
            *c.charisma_modifier = (*c.charisma as i32 - 10) / 2;
        }

        if c.attacks_per_action.initialize(NAME) {
            *c.attacks_per_action = 1;
        }
    }
    pub fn last(_c: &mut Character) {}
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CommonClassContent {
    pub lvl: usize,
    first_class: bool
}

impl CommonClassContent {
    pub fn declare(&self, c: &mut Character, class: &dyn Class) {
        let name = class.name();
        c.max_health.declare_finalizer(name);
        c.total_level.declare_modifier(name);
        c.class_names.declare_modifier(name);
    }
    pub fn iterate(&self, c: &mut Character, class: &dyn Class) {
        let name = class.name();
        let hd = class.hit_dice();
        if c.constitution_modifier.finalized() && c.max_health.finalize(name) {
            let mut res: i32 = 0;
            if self.first_class && self.lvl >= 1 {
                res += (hd / 2 - 1) as i32;
            }
            res += ((hd / 2 + 1) as i32 + *c.constitution_modifier) * self.lvl as i32;
            *c.max_health += res as usize;
        }
        if c.total_level.modify(name) {
            *c.total_level += self.lvl;
        }
        if c.class_names.modify(name) {
            (*c.class_names).push(format!("{} {}", name, self.lvl));
        }
    }
    pub fn last(&mut self, _c: &mut Character) {}
}

pub(crate) mod common_race_rules {
    use crate::character::Character;
    use crate::content::traits::Race;

    pub fn declare(c: &mut Character, race: &dyn Race) {
        c.race_name.declare_initializer(race.name());
    }
    pub fn iterate(c: &mut Character, race: &dyn Race) {
        if c.race_name.initialize(race.name()) {
            *c.race_name = race.name().to_string();
        }
    }
}