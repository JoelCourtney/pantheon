crate::name!("Unknown");

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UnknownItem;

#[content]
impl Item for UnknownItem {
    properties! {
        equipable: Equipable = Equipable::No,
        rarity: Rarity = Rarity::Common,
        weight: Option<u32> = None,
        cost: Option<u32> = None,
        magical: bool = false
    }

    description! {r#"
        # Unknown Item

        Please choose a real item. This is a placeholder.
    "#}
}
