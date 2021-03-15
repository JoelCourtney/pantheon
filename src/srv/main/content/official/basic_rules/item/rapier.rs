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

    fn declare(&self, c: &mut Character, equipped: Equipped, _attuned: bool) {
        if let Equipped::Held(_) = equipped {
            c.attack_moves.declare_initializer(NAME);
        }
    }
    fn iterate(&self, c: &mut Character, equipped: Equipped, _attuned: bool) {
        if c.ability_modifiers.strength.ready() && c.ability_modifiers.dexterity.ready() {
            let ability = if *c.ability_modifiers.strength > *c.ability_modifiers.dexterity {
                Ability::Strength
            } else {
                Ability::Dexterity
            };
            if let Equipped::Held(_) = equipped {
                if c.attack_moves.initialize(NAME) {
                    (*c.attack_moves).push(
                        AttackMove {
                            name: NAME,
                            time: MoveTime::Action,
                            hit: 0,
                            damage: Damage::from_die(8, DamageType::Piercing),
                            range: Range::Fixed(5),
                            properties: vec!["Finesse"],
                            use_modifier: ability,
                            weapon_type: WeaponType::Martial
                        }
                    )
                }
            }
        }
    }

    description! {r#"
        # Rapier

        Proficiency with a rapier allows you to add your proficiency bonus to the attack roll for any attack you make with it.
    "#}
}