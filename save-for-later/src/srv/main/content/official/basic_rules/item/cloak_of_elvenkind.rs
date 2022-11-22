crate::name!("Cloak of Elvenkind");

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CloakOfElvenkind {
    active: bool
}

#[content]
impl Item for CloakOfElvenkind {
    properties! {
        magical, attunable;

        equipable: Equipable = Equipable::Yes,
        rarity: Rarity = Rarity::Uncommon,
        weight: Option<u32> = None,
        cost: Option<u32> = None
    }

    fn resolve(&mut self, c: &mut Character, equipped: Equipped, attuned: bool) {
        if attuned {
            if equipped == Equipped::Yes {
                if self.active {
                    m! { c.skill_vantages.stealth += 1 }
                }
                i! {
                    c.moves <<= Move::Other {
                        element: Element::Toggle {
                            text: if self.active {
                                "**Cloak of Elvenkind:** Pull the hood down to deactivate."
                            } else {
                                "**Cloak of Elvenkind:** Pull the hood up to activate."
                            },
                            data: &mut self.active,
                            button: vec! [
                                if self.active {
                                    "Pull Down"
                                } else {
                                    "Pull Up"
                                }
                            ]
                        },
                        time: MoveTime::Action
                    }
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

