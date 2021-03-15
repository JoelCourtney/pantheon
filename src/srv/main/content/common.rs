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

        c.strength_save.declare_initializer(NAME);
        c.dexterity_save.declare_initializer(NAME);
        c.constitution_save.declare_initializer(NAME);
        c.intelligence_save.declare_initializer(NAME);
        c.wisdom_save.declare_initializer(NAME);
        c.charisma_save.declare_initializer(NAME);

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

        c.passive_perception.declare_initializer(NAME);
        c.passive_investigation.declare_initializer(NAME);
        c.passive_insight.declare_initializer(NAME);

        c.proficiency_bonus.declare_initializer(NAME);
        c.initiative.declare_initializer(NAME);

        c.attacks_per_action.declare_initializer(NAME);

        c.armor_class.declare_initializer(NAME);

        // MODIFIERS

        c.attack_moves.declare_modifier(NAME);
    }
    pub fn iterate(c: &mut Character) {
        // INITIALIZERS

        // ABILITY SCORE MODIFIERS
        if c.strength.ready() && c.strength_modifier.initialize(NAME) {
            *c.strength_modifier = (*c.strength as i32 - 10) / 2;
        }
        if c.dexterity.ready() && c.dexterity_modifier.initialize(NAME) {
            *c.dexterity_modifier = (*c.dexterity as i32 - 10) / 2;
        }
        if c.constitution.ready() && c.constitution_modifier.initialize(NAME) {
            *c.constitution_modifier = (*c.constitution as i32 - 10) / 2;
        }
        if c.intelligence.ready() && c.intelligence_modifier.initialize(NAME) {
            *c.intelligence_modifier = (*c.intelligence as i32 - 10) / 2;
        }
        if c.wisdom.ready() && c.wisdom_modifier.initialize(NAME) {
            *c.wisdom_modifier = (*c.wisdom as i32 - 10) / 2;
        }
        if c.charisma.ready() && c.charisma_modifier.initialize(NAME) {
            *c.charisma_modifier = (*c.charisma as i32 - 10) / 2;
        }

        // PROFICIENCY BONUS
        if c.total_level.ready() && c.proficiency_bonus.initialize(NAME) {
            *c.proficiency_bonus = (*c.total_level - 1) / 4 + 2;
        }

        // INITIATIVE
        if c.dexterity_modifier.ready() && c.initiative.initialize(NAME) {
            *c.initiative = *c.dexterity_modifier;
        }

        // DEFAULT ATTACKS PER ACTION
        if c.attacks_per_action.initialize(NAME) {
            *c.attacks_per_action = 1;
        }

        // DEFAULT ARMOR-LESS AC
        if c.dexterity_modifier.ready() && c.armor_class.initialize(NAME) {
            let default = (10 + *c.dexterity_modifier) as u32;
            if *c.armor_class < default {
                *c.armor_class = default;
            }
        }

        // INIT SKILLS and SAVING THROWS
        if c.proficiency_bonus.ready() {
            if c.strength_modifier.ready() {
                if c.athletics_proficiency.ready()
                    && c.athletics.initialize(NAME) {
                    *c.athletics = *c.strength_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.athletics_proficiency);
                }

                if c.strength_save_proficiency.ready()
                    && c.strength_save.initialize(NAME) {
                    *c.strength_save = *c.strength_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.strength_save_proficiency);
                }
            }

            if c.dexterity_modifier.ready() {
                if c.acrobatics_proficiency.ready()
                    && c.acrobatics.initialize(NAME) {
                    *c.acrobatics = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.acrobatics_proficiency);
                }
                if c.sleight_of_hand_proficiency.ready()
                    && c.sleight_of_hand.initialize(NAME) {
                    *c.sleight_of_hand = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.sleight_of_hand_proficiency);
                }
                if c.stealth_proficiency.ready()
                    && c.stealth.initialize(NAME) {
                    *c.stealth = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.stealth_proficiency);
                }

                if c.dexterity_save_proficiency.ready()
                    && c.dexterity_save.initialize(NAME) {
                    *c.dexterity_save = *c.dexterity_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.dexterity_save_proficiency);
                }
            }

            if c.constitution_modifier.ready() {
                if c.constitution_save_proficiency.ready()
                    && c.constitution_save.initialize(NAME) {
                    *c.constitution_save = *c.constitution_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.constitution_save_proficiency);
                }
            }

            if c.intelligence_modifier.ready() {
                if c.arcana_proficiency.ready()
                    && c.arcana.initialize(NAME) {
                    *c.arcana = *c.intelligence_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.arcana_proficiency);
                }
                if c.history_proficiency.ready()
                    && c.history.initialize(NAME) {
                    *c.history = *c.intelligence_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.history_proficiency);
                }
                if c.investigation_proficiency.ready()
                    && c.investigation.initialize(NAME) {
                    *c.investigation = *c.intelligence_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.investigation_proficiency);
                }
                if c.nature_proficiency.ready()
                    && c.nature.initialize(NAME) {
                    *c.nature = *c.intelligence_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.nature_proficiency);
                }
                if c.religion_proficiency.ready()
                    && c.religion.initialize(NAME) {
                    *c.religion = *c.intelligence_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.religion_proficiency);
                }

                if c.intelligence_save_proficiency.ready()
                    && c.intelligence_save.initialize(NAME) {
                    *c.intelligence_save = *c.intelligence_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.intelligence_save_proficiency);
                }
            }

            if c.wisdom_modifier.ready() {
                if c.animal_handling_proficiency.ready()
                    && c.animal_handling.initialize(NAME) {
                    *c.animal_handling = *c.wisdom_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.animal_handling_proficiency);
                }
                if c.insight_proficiency.ready()
                    && c.insight.initialize(NAME) {
                    *c.insight = *c.wisdom_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.insight_proficiency);
                }
                if c.medicine_proficiency.ready()
                    && c.medicine.initialize(NAME) {
                    *c.medicine = *c.wisdom_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.medicine_proficiency);
                }
                if c.perception_proficiency.ready()
                    && c.perception.initialize(NAME) {
                    *c.perception = *c.wisdom_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.perception_proficiency);
                }
                if c.survival_proficiency.ready()
                    && c.survival.initialize(NAME) {
                    *c.survival = *c.wisdom_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.survival_proficiency);
                }

                if c.wisdom_save_proficiency.ready()
                    && c.wisdom_save.initialize(NAME) {
                    *c.wisdom_save = *c.wisdom_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.wisdom_save_proficiency);
                }
            }

            if c.charisma_modifier.ready() {
                if c.deception_proficiency.ready()
                    && c.deception.initialize(NAME) {
                    *c.deception = *c.charisma_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.deception_proficiency);
                }
                if c.intimidation_proficiency.ready()
                    && c.intimidation.initialize(NAME) {
                    *c.intimidation = *c.charisma_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.intimidation_proficiency);
                }
                if c.performance_proficiency.ready()
                    && c.performance.initialize(NAME) {
                    *c.performance = *c.charisma_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.performance_proficiency);
                }
                if c.persuasion_proficiency.ready()
                    && c.persuasion.initialize(NAME) {
                    *c.persuasion = *c.charisma_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.persuasion_proficiency);
                }

                if c.charisma_save_proficiency.ready()
                    && c.charisma_save.initialize(NAME) {
                    *c.charisma_save = *c.charisma_modifier
                        + calculate_proficiency(*c.proficiency_bonus, *c.charisma_save_proficiency);
                }
            }
        }

        // PASSIVES
        if c.perception.ready() &&
            c.passive_perception.initialize(NAME) {
            *c.passive_perception = 10 + *c.perception;
        }
        if c.investigation.ready() &&
            c.passive_investigation.initialize(NAME) {
            *c.passive_investigation = 10 + *c.investigation;
        }
        if c.insight.ready() &&
            c.passive_insight.initialize(NAME) {
            *c.passive_insight = 10 + *c.insight;
        }

        // MODIFIERS

        // ATTACK SKILL MODIFIERS
        if c.strength_modifier.ready() &&
            c.dexterity_modifier.ready() &&
            c.proficiency_bonus.ready() &&
            c.weapon_proficiencies.ready() &&
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
    pub fn last(c: &mut Character) {
        c.race_choices = crate::content::get_all_race();
    }

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
        if c.constitution_modifier.ready() && c.max_health.initialize(name) {
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