crate::name!("Padded");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Padded;

#[content]
impl Item for Padded {
    properties! {
        equipable: Equipable = Equipable::Armor,
        rarity: Rarity = Rarity::Common,
        weight: Option<u32> = Some(8),
        cost: Option<u32> = Some(5)
    }

    fn declare(&self, c: &mut Character, e: Equipped, _: bool) {
        if e == Equipped::Yes {
            i!(c.armor_class);
            m!(c.skill_vantages.stealth);
        }
    }

    fn iterate(&self, c: &mut Character, e: Equipped, _: bool) {
        if e == Equipped::Yes {
            i! { c.armor_class = 11 + c.ability_modifiers.dexterity? as u32 }
            m! { c.skill_vantages.stealth -= 1 }
        }
    }

    description! {r#"
        # Padded

        Padded armor consists of quilted layers of cloth and batting.
    "#}
}