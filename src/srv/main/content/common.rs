pub(crate) mod common_rules {
    use crate::character::Character;
    use crate::misc::{Ability, ProficiencyType, Skill, PassiveSkill};
    use enum_iterator::IntoEnumIterator;
    use proc_macros::{i, unique_id};
    use crate::moves::Move;

    pub fn resolve(c: &mut Character) {
        // INITIALIZERS

        // ABILITY SCORE MODIFIERS
        for ability in Ability::into_enum_iter() {
            if ability.known() {
                i! {
                    *c.ability_modifiers.get_mut_known(ability) = (c.abilities.get_known(ability)? as i32 / 2)  - 5
                }
            }
        }

        i! {
            c.proficiency_bonus = {
                let level = c.total_level?;
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
        let id = unique_id!();
        if c.moves.request_modify(id) {
            let mut ready = true;
            for r#move in &*c.moves {
                if let Move::Attack { use_modifier, ..} = r#move {
                    if let Some(modifier) = c.ability_modifiers.get(*use_modifier) {
                        if !modifier.finalized() {
                            ready = false;
                            break;
                        }
                    }
                }
            }

            if ready &&
                c.proficiency_bonus.finalized() &&
                c.weapon_proficiencies.finalized() {
                for r#move in &mut *c.moves {
                    match r#move {
                        Move::Attack {
                            name,
                            use_modifier,
                            hit,
                            ..
                        } => {
                            if let Some(modifier) = c.ability_modifiers.get(*use_modifier) {
                                *hit += **modifier
                            }
                            if (*c.weapon_proficiencies).contains(name) {
                                *hit += *c.proficiency_bonus as i32;
                            }
                        }
                        _ => {}
                    }
                }
                c.moves.confirm_modify(id);
            }
        }
        i! {
            c.race_choices = crate::content::get_all_race();
            c.background_choices = crate::content::get_all_background();
            c.class_choices = crate::content::get_all_class();
        }
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
    use proc_macros::i;

    pub fn resolve(c: &mut Character, class: &Box<dyn Class>, level: u32, index: usize) {
        let hd = class.hit_dice();
        i! {
            index;
            c.max_health += {
                let mut res: i32 = 0;
                if index == 0 {
                    res += (hd / 2 - 1) as i32;
                }
                res += ((hd / 2 + 1) as i32 + c.ability_modifiers.constitution?) * level as i32;
                res as u32
            };
            c.class_names <<= class.name().to_string();
            c.class_levels <<= level;
            c.total_level += level
        }
    }
}

pub(crate) mod common_race_rules {
    use crate::character::Character;
    use crate::content::traits::Race;
    use proc_macros::i;

    pub fn resolve(c: &mut Character, race: &Box<dyn Race>) {
        i! { c.race_name = race.name().to_string() }
    }
}

pub(crate) mod common_background_rules {
    use crate::character::Character;
    use crate::content::traits::Background;
    use proc_macros::i;

    pub fn resolve(c: &mut Character, background: &Box<dyn Background>) {
        i! { c.background_name = background.name().to_string()}
    }
}

pub(crate) mod common_item_rules {
    use crate::content::traits::Item;
    use proc_macros::i;
    use crate::misc::{Equipped, Equipable, Holdable, Hand};
    use crate::character::Character;

    pub fn resolve(c: &mut Character, item: &Box<dyn Item>, equipped: Equipped, _attuned: bool) {
        match item.equipable() {
            Equipable::Armor => {
                match equipped {
                    Equipped::Yes => i! { c.armor = Some(item.name()) },
                    Equipped::No => i! {
                        item.name();
                        c.armor_choices <<= item.name();
                    },
                    Equipped::Held(_) => panic!("armor isn't holdable")
                }
            }
            Equipable::Holdable(hold) => {
                match hold {
                    Holdable::Ammunition => {
                        match equipped {
                            Equipped::Yes => i! { c.ammunition = Some(item.name()) },
                            Equipped::No => i! {
                                item.name();
                                c.ammunition_choices <<= item.name();
                            },
                            Equipped::Held(_) => panic!("ammunition isn't holdable")
                        }
                    }
                    _ => {
                        match equipped {
                            Equipped::Held(hand) => {
                                match hand {
                                    Hand::Left => i! { c.left_hand = Some(item.name()) },
                                    Hand::Right => i! { c.right_hand = Some(item.name()) },
                                    Hand::Both => i! { c.both_hands = Some(item.name()) }
                                }
                            }
                            Equipped::No => i! {
                                item.name();
                                c.hold_choices <<= item.name();
                            },
                            Equipped::Yes => panic!("holdables aren't generically equippable")
                        }
                    }
                }
            }
            _ => {}
        }
    }
}