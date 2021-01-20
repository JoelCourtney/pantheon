#[macros::custom_content("HalflingSubrace")]
pub struct Lightfoot;

impl Content for Lightfoot {
    fn declare(&self, c: &mut Character) {
        c.charisma.declare_modifier(NAME);
    }
    fn iterate(&self, c: &mut Character) {
        if c.charisma.modify(NAME) {
            *c.charisma += 1;
        }
    }

    fn last(&mut self, c: &mut Character) {
        c.race_traits.extend(vec![
            Feature (
                indoc! {r"
                    # Ability Score Increase

                    Your Charisma score increases by 1.
                "},
                None
            ),
            Feature (
                indoc! {r"
                    # Naturally Stealthy

                    You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.
                "},
                None
            )
        ]);
    }
}

describe! {r#"
    # Lightfoot

    As a lightfoot halfling, you can easily hide from notice, even using other people as cover. You’re inclined to be affable and get along well with others. In the Forgotten Realms, lightfoot halflings have spread the farthest and thus are the most common variety.

    Lightfoots are more prone to wanderlust than other halflings, and often dwell alongside other races or take up a nomadic life. In the world of Greyhawk, these halflings are called hairfeet or tallfellows.

    ## Ability Score Increase

    Your Charisma score increases by 1.

    ## Naturally Stealthy

    You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.
"#}