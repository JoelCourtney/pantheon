crate::name!("Stout");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Stout;

#[content]
impl HalflingSubrace for Stout {
    fn resolve(&mut self, c: &mut Character) {
        i! {
            c.saving_throw_notes <<= "**ADV** against poisoned";
            c.defenses <<= "**RES** poison";
        }
        m! { c.abilities.constitution += 1 }
        i! {c.race_traits >>= vec![
            Feature (
                "**Ability Score Increase:** Your Constitution score increases by 1.",
                Empty
            ),
            Feature (
                "**Stout Resilience:** You have advantage on saving throws against poison, and you have resistance against poison damage.",
                Empty
            )
        ];
        }
    }

    description! {r#"
        # Stout

        As a stout halfling, you’re hardier than average and have some resistance to poison. Some say that stouts have dwarven blood. In the Forgotten Realms, these halflings are called stronghearts, and they’re most common in the south.

        ## Ability Score Increase

        Your Constitution score increases by 1.

        ## Stout Resilience

        You have advantage on saving throws against poison, and you have resistance against poison damage.
    "#}
}

