#[macros::race("Unknown Race")]
pub struct UnknownRace;

impl Content for UnknownRace {}

describe! { r#"
    # Unknown Race

    Please choose a race. This is a placeholder.
"#}