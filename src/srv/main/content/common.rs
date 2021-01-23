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

        c.proficiency_bonus.declare_initializer(NAME);
        c.initiative.declare_initializer(NAME);
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

        if c.total_level.finalized() && c.proficiency_bonus.initialize(NAME) {
            *c.proficiency_bonus = (*c.total_level - 1) / 4 + 2;
        }
        if c.dexterity_modifier.finalized() && c.initiative.initialize(NAME) {
            *c.initiative = *c.dexterity_modifier;
        }
    }
    pub fn last(_c: &mut Character) {}
}

pub(crate) mod common_class_rules {
    use crate::character::Character;
    use crate::content::traits::Class;

    pub fn declare(class: &dyn Class, c: &mut Character, _level: u32, _first: bool) {
        let name = class.name();
        c.max_health.declare_initializer(name);
        c.total_level.declare_modifier(name);
        c.class_names.declare_modifier(name);
    }
    pub fn iterate(class: &dyn Class, c: &mut Character, level: u32, first: bool) {
        let name = class.name();
        let hd = class.hit_dice();
        if c.constitution_modifier.finalized() && c.max_health.initialize(name) {
            let mut res: i32 = 0;
            if first && level >= 1 {
                res += (hd / 2 - 1) as i32;
            }
            res += ((hd / 2 + 1) as i32 + *c.constitution_modifier) * level as i32;
            *c.max_health += res as u32;
        }
        if c.total_level.modify(name) {
            *c.total_level += level;
        }
        if c.class_names.modify(name) {
            (*c.class_names).push(format!("{} {}", name, level));
        }
    }
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