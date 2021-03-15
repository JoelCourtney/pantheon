crate::name!("Stout");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Stout;

#[content]
impl HalflingSubrace for Stout {
    properties! {}

    fn declare(&self, c: &mut Character) {
        c.abilities.constitution.declare_modifier(NAME);
        c.saving_throw_notes.declare_initializer(NAME);
        c.defenses.declare_initializer(NAME);
    }
    fn iterate(&self, c: &mut Character) {
        if c.abilities.constitution.modify(NAME) {
            *c.abilities.constitution += 1;
        }
        if c.saving_throw_notes.initialize(NAME) {
            (*c.saving_throw_notes).push("**ADV** against poisoned");
        }
        if c.defenses.initialize(NAME) {
            (c.defenses).push("**RES** poison");
        }
    }

    fn last(&mut self, c: &mut Character) {
        c.race_traits.extend(vec![
            Feature (
                "**Ability Score Increase:** Your Constitution score increases by 1.",
                Empty
            ),
            Feature (
                "**Stout Resilience:** You have advantage on saving throws against poison, and you have resistance against poison damage.",
                Empty
            )
        ]);
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

