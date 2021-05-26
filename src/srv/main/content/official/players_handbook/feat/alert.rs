crate::name!("Alert");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Alert;

#[content]
impl Feat for Alert {
    fn resolve(&mut self, c: &mut Character) {
        m! { c.initiative += 5 }
        i! {
            c.feats <<= Element::Str (
                indoc! { "
                    **Alert:** Always on the lookout for danger, you gain the following benefits:

                    - You gain a `+5` bonus to `initiative`.
                    - You can't be surprised while you are conscious.
                    - Other creatures don't gain advantage on attack rolls against you as a result of being unseen by you.
                " }
            );
        }
    }

    description! { r#"
        # Alert

        Always on the lookout for danger, you gain the following benefits:

        - You gain a +5 bonus to initiative.
        - You can't be surprised while you are conscious.
        - Other creatures don't gain advantage on attack rolls against you as a result of being unseen by you.
    "# }
}

