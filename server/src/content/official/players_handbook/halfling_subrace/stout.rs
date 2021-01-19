#[macros::custom_content("HalflingSubrace")]
pub struct Stout;

impl Content for Stout {
    fn declare(&self, c: &mut Character) {
        c.constitution.declare_modifier(NAME);
    }
    fn iterate(&self, c: &mut Character) {
        if c.constitution.modify(NAME) {
            *c.constitution += 1;
        }
    }

    // fn features(&self) -> Vec<Feature> {
    //     vec![
    //         Feature (
    //             indoc! {r"
    //                 # Ability Score Increase
    //
    //                 Your Constitution score increases by 1.
    //             "},
    //             None
    //         ),
    //         Feature (
    //             indoc! {r"
    //                 # Stout Resilience
    //
    //                 You have advantage on saving throws against poison, and you have resistance against poison damage.
    //             "},
    //             None
    //         )
    //     ]
    // }
}

describe! {r#"
    # Stout

    As a stout halfling, you’re hardier than average and have some resistance to poison. Some say that stouts have dwarven blood. In the Forgotten Realms, these halflings are called stronghearts, and they’re most common in the south.

    ## Ability Score Increase

    Your Constitution score increases by 1.

    ## Stout Resilience

    You have advantage on saving throws against poison, and you have resistance against poison damage.
"#}