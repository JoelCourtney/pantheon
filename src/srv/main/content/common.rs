pub(crate) mod common_rules {
    use crate::character::Character;
    use crate::misc::{Ability, ProficiencyType, Skill, PassiveSkill};
    use enum_iterator::IntoEnumIterator;
    use proc_macros::i;

    const NAME: &'static str = "Common Rules";

    pub fn iterate(c: &mut Character) {
        // INITIALIZERS

        // ABILITY SCORE MODIFIERS
        for ability in Ability::into_enum_iter() {
            if ability.known() {
                i! {
                    *c.ability_modifiers.get_mut_known(ability) = (c.abilities.get_known(ability)? as i32 - 10) / 2
                }
            }
        }

        i! {
            c.proficiency_bonus = {
                let level = c.total_level?;
                dbg!(level);
                if level >= 1 && level <= 20 {
                    (level - 1) / 4 + 2
                } else {
                    0
                }
            };
            c.initiative = c.ability_modifiers.dexterity?;
            c.attacks_per_action = 1;
            c.armor_class = {
                let default = (10 + c.ability_modifiers.dexterity?) as u32;
                if *c.armor_class < default {
                    default
                } else {
                    *c.armor_class
                }
            };
        }

        // INIT SKILLS and SAVING THROWS
        for skill in Skill::into_enum_iter() {
            let ability = skill.get_associated_ability();
            if skill.known() {
                i! {
                    *c.skills.get_mut_known(skill) = c.ability_modifiers.get_known(ability)?
                        + calculate_proficiency(c.proficiency_bonus?, c.skill_proficiencies.get_known(skill)?)
                }
            }
        }
        for ability in Ability::into_enum_iter() {
            if ability.known() {
                i! {
                    *c.saves.get_mut_known(ability) = c.ability_modifiers.get_known(ability)?
                        + calculate_proficiency(c.proficiency_bonus?, c.save_proficiencies.get_known(ability)?)
                }
            }
        }

        // PASSIVES
        for passive in PassiveSkill::into_enum_iter() {
            if passive.known() {
                i! { *c.passives.get_mut_known(passive) = 10 + c.skills.get_known(passive.into_skill())? }
            }
        }

        // MODIFIERS

        // ATTACK SKILL MODIFIERS
        if c.attack_moves.request_modify(NAME) {
            if c.ability_modifiers.strength.finalized() &&
                c.ability_modifiers.dexterity.finalized() &&
                c.proficiency_bonus.finalized() &&
                c.weapon_proficiencies.finalized() {
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
                c.attack_moves.confirm_modify(NAME);
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
    use proc_macros::{i, m};

    pub fn iterate(class: &dyn Class, c: &mut Character, level: u32, first: bool) {
        #[allow(non_snake_case)] let NAME = class.name();
        let hd = class.hit_dice();
        i! {
            c.max_health = {
                let mut res: i32 = 0;
                if first && level >= 1 {
                    res += (hd / 2 - 1) as i32;
                }
                res += ((hd / 2 + 1) as i32 + c.ability_modifiers.constitution?) * level as i32;
                res as u32
            };
            c.class_names <<= format!("{} {}", NAME, level);
        }
        m! { c.total_level += dbg!(level) }
    }
    pub fn last(_class: &dyn Class, _c: &mut Character, _level: u32, _first: bool) {

    }
}

pub(crate) mod common_race_rules {
    use crate::character::Character;
    use crate::content::traits::Race;
    use proc_macros::i;

    pub fn iterate(c: &mut Character, race: &dyn Race) {
        #[allow(non_snake_case)]
        let NAME = race.name();

        i! { c.race_name = race.name().to_string() }
    }
}