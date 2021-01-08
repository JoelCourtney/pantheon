#[macros::race]
pub struct UnknownRace;

impl Modify for UnknownRace {}
impl Featured for UnknownRace {}

describe! { r#"
    # Unknown Race

    Please choose a race. This is a placeholder.
"#}