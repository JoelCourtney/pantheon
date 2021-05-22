crate::name!("Rapier");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rapier;

#[content]
impl Item for Rapier {
    properties! {
        equipable: Equipable = Equipable::Holdable(Holdable::One),
        rarity: Rarity = Rarity::Common,
        weight: Option<u32> = Some(2),
        cost: Option<u32> = Some(25)
    }

    fn resolve(&mut self, c: &mut Character, equipped: Equipped, _attuned: bool) {
        if let Equipped::Held(_) = equipped {
            i! {
                c.moves <<= Move::Attack {
                    name: name!(),
                    time: MoveTime::Action,
                    hit: 0,
                    damage: Damage::from_die(8, DamageType::Piercing),
                    range: Range::Fixed(5),
                    properties: vec!["Finesse"],
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
        # Rapier

        Proficiency with a rapier allows you to add your proficiency bonus to the attack roll for any attack you make with it.
    "#}
}