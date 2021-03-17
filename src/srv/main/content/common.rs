pub(crate) mod common_rules {
    use crate::character::Character;
    use crate::misc::{Ability, ProficiencyType, Skill, PassiveSkill};
    use enum_iterator::IntoEnumIterator;

    const NAME: &'static str = "Common Rules";

    pub fn declare(c: &mut Character) {
        // INITIALIZERS

        for ability in Ability::into_enum_iter() {
            if ability.known() {
                c.ability_modifiers.get_mut_known(ability).declare_initializer(NAME);
                c.saves.get_mut_known(ability).declare_initializer(NAME);
            }
        }

        for skill in Skill::into_enum_iter() {
            if skill.known() {
                c.skills.get_mut_known(skill).declare_initializer(NAME);
            }
        }

        for passive in PassiveSkill::into_enum_iter() {
            if passive.known() {
                c.passives.get_mut_known(passive).declare_initializer(NAME);
            }
        }

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
        for ability in Ability::into_enum_iter() {
            if ability.known()
                && c.abilities.get_known(ability).ready()
                && c.ability_modifiers.get_mut_known(ability).initialize(NAME) {
                **c.ability_modifiers.get_mut_known(ability) = (**c.abilities.get_known(ability) as i32 - 10) / 2;
            }
        }
        // if c.abilities.strength.ready() && c.strength_modifier.initialize(NAME) {
        //     *c.strength_modifier = (*c.abilities.strength as i32 - 10) / 2;
        // }

        // PROFICIENCY BONUS
        if c.total_level.ready() && c.proficiency_bonus.initialize(NAME) {
            *c.proficiency_bonus = (*c.total_level - 1) / 4 + 2;
        }

        // INITIATIVE
        if c.ability_modifiers.dexterity.ready() && c.initiative.initialize(NAME) {
            *c.initiative = *c.ability_modifiers.dexterity;
        }

        // DEFAULT ATTACKS PER ACTION
        if c.attacks_per_action.initialize(NAME) {
            *c.attacks_per_action = 1;
        }

        // DEFAULT ARMOR-LESS AC
        if c.ability_modifiers.dexterity.ready() && c.armor_class.initialize(NAME) {
            let default = (10 + *c.ability_modifiers.dexterity) as u32;
            if *c.armor_class < default {
                *c.armor_class = default;
            }
        }

        // INIT SKILLS and SAVING THROWS
        if c.proficiency_bonus.ready() {
            for skill in Skill::into_enum_iter() {
                let ability = skill.get_associated_ability();
                if skill.known()
                    && c.ability_modifiers.get_known(ability).ready()
                    && c.skill_proficiencies.get_known(skill).ready()
                    && c.skills.get_mut_known(skill).initialize(NAME) {
                    **c.skills.get_mut_known(skill) = **c.ability_modifiers.get_known(ability)
                        + calculate_proficiency(*c.proficiency_bonus, **c.skill_proficiencies.get_known(skill));
                }
            }
            for ability in Ability::into_enum_iter() {
                if ability.known()
                    && c.ability_modifiers.get_known(ability).ready()
                    && c.save_proficiencies.get_known(ability).ready()
                    && c.saves.get_mut_known(ability).initialize(NAME) {
                    **c.saves.get_mut_known(ability) = **c.ability_modifiers.get_known(ability)
                        + calculate_proficiency(*c.proficiency_bonus, **c.save_proficiencies.get_known(ability));
                }
            }
        }

        // PASSIVES
        for passive in PassiveSkill::into_enum_iter() {
            if passive.known()
                && c.skills.get_known(passive.into_skill()).ready()
                && c.passives.get_mut_known(passive).initialize(NAME) {
                **c.passives.get_mut_known(passive) = 10 + **c.skills.get_known(passive.into_skill());
            }
        }

        // MODIFIERS

        // ATTACK SKILL MODIFIERS
        if c.ability_modifiers.strength.ready() &&
            c.ability_modifiers.dexterity.ready() &&
            c.proficiency_bonus.ready() &&
            c.weapon_proficiencies.ready() &&
            c.attack_moves.modify(NAME) {
            for attack in &mut *c.attack_moves {
                match attack.use_modifier {
                    Ability::Strength => attack.hit += *c.ability_modifiers.strength,
                    Ability::Dexterity => attack.hit += *c.ability_modifiers.dexterity,
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
        if c.ability_modifiers.constitution.ready() && c.max_health.initialize(name) {
            let mut res: i32 = 0;
            if first && level >= 1 {
                res += (hd / 2 - 1) as i32;
            }
            res += ((hd / 2 + 1) as i32 + *c.ability_modifiers.constitution) * level as i32;
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