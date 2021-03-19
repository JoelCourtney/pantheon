crate::name!("Unknown");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownRace;

#[content]
impl Race for UnknownRace {
    properties! {}

    fn iterate(&self, c: &mut Character) {
        common_race_rules::iterate(c, self);
    }

    description! { r#"
        # Unknown Race

        Please choose a race. This is a placeholder.
    "#}
}

