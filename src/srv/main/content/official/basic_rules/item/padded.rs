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
            c.armor_class.declare_initializer(NAME);
            c.stealth_vantage.declare_modifier(NAME);
        }
    }

    fn iterate(&self, c: &mut Character, e: Equipped, _: bool) {
        if e == Equipped::Yes {
            if c.dexterity_modifier.finalized() && c.armor_class.initialize(NAME) {
                *c.armor_class = 11 + *c.dexterity_modifier as u32;
            }
            if c.stealth_vantage.modify(NAME) {
                c.stealth_vantage.downgrade();
            }
        }
    }

    description! {r#"
        # Padded

        Padded armor consists of quilted layers of cloth and batting.
    "#}
}