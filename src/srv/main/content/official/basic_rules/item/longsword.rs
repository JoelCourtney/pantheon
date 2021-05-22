crate::name!("Longsword");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Longsword;

#[content]
impl Item for Longsword {
    properties! {
        equipable: Equipable = Equipable::Holdable(Holdable::Versatile),
        rarity: Rarity = Rarity::Common,
        weight: Option<u32> = Some(3),
        cost: Option<u32> = Some(15)
    }

    fn resolve(&mut self, c: &mut Character, equipped: Equipped, _attuned: bool) {
        match equipped {
            Equipped::Held(Hand::Both) => {
                i! {
                    c.moves <<= Move::Attack {
                        name: name!(),
                        time: MoveTime::Action,
                        hit: 0,
                        damage: Damage::from_die(10, DamageType::Slashing),
                        range: Range::Fixed(5),
                        properties: vec![ "Versatile" ],
                        use_modifier: Ability::Strength,
                        weapon_type: WeaponType::Martial
                    }
                }
            }
            Equipped::Held(_) => {
                i! {
                    c.moves <<= Move::Attack {
                        name: name!(),
                        time: MoveTime::Action,
                        hit: 0,
                        damage: Damage::from_die(8, DamageType::Slashing),
                        range: Range::Fixed(5),
                        properties: vec![ "Versatile" ],
                        use_modifier: Ability::Strength,
                        weapon_type: WeaponType::Martial
                    }
                }
            }
            _ => {}
        }
    }
    description! {r#"
        # Longsword

        Proficiency with a longsword allows you to add your proficiency bonus to the attack roll for any attack you make with it.
    "#}
}