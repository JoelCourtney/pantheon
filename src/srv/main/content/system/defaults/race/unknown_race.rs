crate::name!("Unknown Race");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownRace;

#[content]
impl Race for UnknownRace {
    properties! {}

    description! { r#"
        # Unknown Race

        Please choose a race. This is a placeholder.
    "#}
}

