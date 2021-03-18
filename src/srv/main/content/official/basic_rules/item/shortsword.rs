use crate::misc::Ability;
crate::name!("Shortsword");

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Shortsword;

#[content]
impl Item for Shortsword {
    properties! {
        equipable: Equipable = Equipable::Holdable(Holdable::Versatile),
        weight: Option<u32> = Some(2),
        cost: Option<u32> = Some(10)
    }

    fn declare(&self, c: &mut Character, equipped: Equipped, _attuned: bool) {
        if let Equipped::Held(_) = equipped {
            i!(c.attack_moves);
        }
    }

    fn iterate(&self, c: &mut Character, equipped: Equipped, _attuned: bool) {
        if c.ability_modifiers.strength.ready() && c.ability_modifiers.dexterity.ready() {
            let ability = if *c.ability_modifiers.strength > *c.ability_modifiers.dexterity {
                Ability::Strength
            } else {
                Ability::Dexterity
            };
            match equipped {
                Equipped::Held(_) => {
                    i! {
                        c.attack_moves <<= AttackMove {
                            name: NAME,
                            time: MoveTime::Action,
                            hit: 0,
                            damage: Damage::from_die(6, DamageType::Piercing),
                            range: Range::Fixed(5),
                            properties: vec!["Light", "Finesse"],
                            use_modifier: ability,
                            weapon_type: WeaponType::Martial
                        }
                    }
                }
                _ => {}
            }
        }
    }
    description! {r#"
        # Shortsword

        Proficiency with a shortsword allows you to add your proficiency bonus to the attack roll for any attack you make with it.
    "#}
}