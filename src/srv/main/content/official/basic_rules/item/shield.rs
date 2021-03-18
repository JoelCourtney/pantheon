crate::name!("Shield");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Shield;

#[content]
impl Item for Shield {
    properties! {
        equipable: Equipable = Equipable::Holdable(Holdable::One),
        rarity: Rarity = Rarity::Common,
        weight: Option<u32> = Some(6),
        cost: Option<u32> = Some(10)
    }

    fn declare(&self, c: &mut Character, e: Equipped, _: bool) {
        match e {
            Equipped::Held(_) => m!(c.armor_class),
            _ => {}
        }
    }

    fn iterate(&self, c: &mut Character, e: Equipped, _: bool) {
        match e {
            Equipped::Held(_) => {
                m! { c.armor_class += 2 }
            }
            _ => {}
        }
    }

    description! {r#"
        # Shield

        A shield is made from wood or metal and is carried in one hand. Wielding a shield increases your Armor Class by 2. You can benefit from only one shield at a time.
    "#}
}