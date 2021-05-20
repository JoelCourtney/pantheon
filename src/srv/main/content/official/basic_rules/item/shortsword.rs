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

    fn resolve(&mut self, c: &mut Character, equipped: Equipped, _attuned: bool) {
        if let Equipped::Held(_) = equipped {
            i! {
                c.moves <<= Move::Attack {
                    name: NAME,
                    time: MoveTime::Action,
                    hit: 0,
                    damage: Damage::from_die(6, DamageType::Piercing),
                    range: Range::Fixed(5),
                    properties: vec!["Light", "Finesse"],
                    use_modifier: {
                        if c.ability_modifiers.strength? > c.ability_modifiers.dexterity? {
                            Ability::Strength
                        } else {
                            Ability::Dexterity
                        }
                    },
                    weapon_type: WeaponType::Martial
                }
            }
        }
    }
    description! {r#"
        # Shortsword

        Proficiency with a shortsword allows you to add your proficiency bonus to the attack roll for any attack you make with it.
    "#}
}