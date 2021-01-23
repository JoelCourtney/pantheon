crate::name!("Cloak of Elvenkind");

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CloakOfElvenkind {

}

#[content]
impl Item for CloakOfElvenkind {
    properties! {
        magical, attunement;

        equipable: Equipable = Equipable::Yes,
        rarity: Rarity = Rarity::Uncommon,
        weight: Option<u32> = None,
        cost: Option<u32> = None
    }
    
    description! {r#"
        # Cloak of Elvenkind

        *Wondrous Item, Uncommon (requires attunement)*

        While you wear this cloak with its hood up, Wisdom (Perception) checks made to see you have disadvantage, and you have advantage on Dexterity (Stealth) checks made to hide, as the cloak's color shifts to camouflage you. Pulling the hood up or down requires an action.

        *Notes: Advantage: Stealth, Deception, Outerwear*
    "#}
}

