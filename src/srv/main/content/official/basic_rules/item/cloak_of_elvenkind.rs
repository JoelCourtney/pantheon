crate::name!("Cloak of Elvenkind");

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CloakOfElvenkind;

#[content]
impl Item for CloakOfElvenkind {
    properties! {
        magical, attunable;

        equipable: Equipable = Equipable::Yes,
        rarity: Rarity = Rarity::Uncommon,
        weight: Option<u32> = None,
        cost: Option<u32> = None
    }

    fn declare(&self, c: &mut Character, equipped: Equipped, attuned: bool) {
        if attuned {
            if equipped == Equipped::Yes {
                c.stealth_vantage.declare_modifier(NAME);
            }
            c.misc_moves.declare_initializer(NAME);
        }
    }

    fn iterate(&self, c: &mut Character, equipped: Equipped, attuned: bool) {
        if attuned {
            if equipped == Equipped::Yes {
                if c.stealth_vantage.modify(NAME) {
                    c.stealth_vantage.upgrade();
                }
                if c.misc_moves.initialize(NAME) {
                    (*c.misc_moves).push(
                        MiscMove {
                            name: "Cloak of Elvenkind",
                            description: "Pulling the hood down requires an action.",
                            time: MoveTime::Action
                        }
                    );
                }
            } else {
                if c.misc_moves.initialize(NAME) {
                    (*c.misc_moves).push(
                        MiscMove {
                            name: "Cloak of Elvenkind",
                            description: "Pulling the hood up requires an action.",
                            time: MoveTime::Action
                        }
                    );
                }
            }
        }
    }

    description! {r#"
        # Cloak of Elvenkind

        *Wondrous Item, Uncommon (requires attunement)*

        While you wear this cloak with its hood up, Wisdom (Perception) checks made to see you have disadvantage, and you have advantage on Dexterity (Stealth) checks made to hide, as the cloak's color shifts to camouflage you. Pulling the hood up or down requires an action.

        *Notes: Advantage: Stealth, Deception, Outerwear*
    "#}
}

