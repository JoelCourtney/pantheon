#[macros::race("Unknown Race")]
pub struct UnknownRace;

#[typetag::serde]
impl Race for UnknownRace {
    properties! {}
}

describe! { r#"
    # Unknown Race

    Please choose a race. This is a placeholder.
"#}