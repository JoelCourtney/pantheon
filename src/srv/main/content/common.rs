pub(crate) mod common_rules {
    use crate::character::Character;
    use crate::misc::{Ability, ProficiencyType};

    const NAME: &'static str = "Common Rules";

    pub fn declare(c: &mut Character) {
        // INITIALIZERS

        c.strength_modifier.declare_initializer(NAME);
        c.dexterity_modifier.declare_initializer(NAME);
        c.constitution_modifier.declare_initializer(NAME);
        c.intelligence_modifier.declare_initializer(NAME);
        c.wisdom_modifier.declare_initializer(NAME);
        c.charisma_modifier.declare_initializer(NAME);

        c.acrobatics.declare_initializer(NAME);
        c.animal_handling.declare_initializer(NAME);
        c.arcana.declare_initializer(NAME);
        c.athletics.declare_initializer(NAME);
        c.deception.declare_initializer(NAME);
        c.history.declare_initializer(NAME);
        c.insight.declare_initializer(NAME);
        c.intimidation.declare_initializer(NAME);
        c.investigation.declare_initializer(NAME);
        c.medicine.declare_initializer(NAME);
        c.nature.declare_initializer(NAME);
        c.perception.declare_initializer(NAME);
        c.performance.declare_initializer(NAME);
        c.persuasion.declare_initializer(NAME);
        c.religion.declare_initializer(NAME);
        c.sleight_of_hand.declare_initializer(NAME);
        c.stealth.declare_initializer(NAME);
        c.survival.declare_initializer(NAME);

        c.proficiency_bonus.declare_initializer(NAME);
        c.initiative.declare_initializer(NAME);

        c.attacks_per_action.declare_initializer(NAME);

        c.armor_class.declare_initializer(NAME);

        // MODIFIERS

        c.attack_moves.declare_modifier(NAME);
    }
    pub fn iterate(c: &mut Character) {
        // INITIALIZERS

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

        if c.total_level.finalized() && c.proficiency_bonus.initialize(NAME) {
            *c.proficiency_bonus = (*c.total_level - 1) / 4 + 2;
        }
        if c.dexterity_modifier.finalized() && c.initiative.initialize(NAME) {
            *c.initiative = *c.dexterity_modifier;
        }

        if c.attacks_per_action.initialize(NAME) {
            *c.attacks_per_action = 1;
        }

        if c.dexterity_modifier.finalized() && c.armor_class.initialize(NAME) {
            let default = (10 + *c.dexterity_modifier) as u32;
            if *c.armor_class < default {
                *c.armor_class = default;
            }
        }

        if c.proficiency_bonus.finalized() {
            if c.dexterity_modifier.finalized() {
                if c.acrobatics_proficiency.finalized()
                    && c.acrobatics.initialize(NAME) {
                    *c.acrobatics = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.acrobatics_proficiency);
                }
                if c.sleight_of_hand_proficiency.finalized()
                    && c.sleight_of_hand.initialize(NAME) {
                    *c.sleight_of_hand = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.sleight_of_hand_proficiency);
                }
                if c.stealth_proficiency.finalized()
                    && c.stealth.initialize(NAME) {
                    *c.stealth = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.stealth_proficiency);
                }
            }

            if c.strength_modifier.finalized() {
                if c.athletics_proficiency.finalized()
                    && c.athletics.initialize(NAME) {
                    *c.athletics = *c.strength_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.athletics_proficiency);
                }
            }

            if c.intelligence_modifier.finalized() {
                if c.arcana_proficiency.finalized()
                    && c.arcana.initialize(NAME) {
                    *c.arcana = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.arcana_proficiency);
                }
                if c.history_proficiency.finalized()
                    && c.history.initialize(NAME) {
                    *c.history = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.history_proficiency);
                }
                if c.investigation_proficiency.finalized()
                    && c.investigation.initialize(NAME) {
                    *c.investigation = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.investigation_proficiency);
                }
                if c.nature_proficiency.finalized()
                    && c.nature.initialize(NAME) {
                    *c.nature = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.nature_proficiency);
                }
                if c.religion_proficiency.finalized()
                    && c.religion.initialize(NAME) {
                    *c.religion = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.religion_proficiency);
                }
            }

            if c.wisdom_modifier.finalized() {
                if c.animal_handling_proficiency.finalized()
                    && c.animal_handling.initialize(NAME) {
                    *c.animal_handling = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.animal_handling_proficiency);
                }
                if c.insight_proficiency.finalized()
                    && c.insight.initialize(NAME) {
                    *c.insight = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.insight_proficiency);
                }
                if c.medicine_proficiency.finalized()
                    && c.medicine.initialize(NAME) {
                    *c.medicine = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.medicine_proficiency);
                }
                if c.perception_proficiency.finalized()
                    && c.perception.initialize(NAME) {
                    *c.perception = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.perception_proficiency);
                }
                if c.survival_proficiency.finalized()
                    && c.survival.initialize(NAME) {
                    *c.survival = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.survival_proficiency);
                }
            }

            if c.charisma_modifier.finalized() {
                if c.deception_proficiency.finalized()
                    && c.deception.initialize(NAME) {
                    *c.deception = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.deception_proficiency);
                }
                if c.intimidation_proficiency.finalized()
                    && c.intimidation.initialize(NAME) {
                    *c.intimidation = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.intimidation_proficiency);
                }
                if c.performance_proficiency.finalized()
                    && c.performance.initialize(NAME) {
                    *c.performance = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.performance_proficiency);
                }
                if c.persuasion_proficiency.finalized()
                    && c.persuasion.initialize(NAME) {
                    *c.persuasion = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.persuasion_proficiency);
                }
            }
        }

        // MODIFIERS

        if c.strength_modifier.finalized() &&
            c.dexterity_modifier.finalized() &&
            c.proficiency_bonus.finalized() &&
            c.weapon_proficiencies.finalized() &&
            c.attack_moves.modify(NAME) {
            for attack in &mut *c.attack_moves {
                match attack.use_modifier {
                    Ability::Strength => attack.hit += *c.strength_modifier,
                    Ability::Dexterity => attack.hit += *c.dexterity_modifier,
                    _ => panic!("unsupported modifier")
                }
                if (*c.weapon_proficiencies).contains(&attack.name) {
                    attack.hit += *c.proficiency_bonus as i32;
                }
            }
        }
    }
    pub fn last(_c: &mut Character) {}

    fn calculate_proficiency(bonus: u32, proficiency: ProficiencyType) -> i32 {
        match proficiency {
            ProficiencyType::None => 0,
            ProficiencyType::Half => bonus as i32 / 2,
            ProficiencyType::Single => bonus as i32,
            ProficiencyType::Double => bonus as i32 * 2
        }
    }
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
    pub fn last(_class: &dyn Class, _c: &mut Character, _level: u32, _first: bool) {

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