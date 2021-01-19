#[macros::race("Unknown Race")]
pub struct UnknownRace;

impl Content for UnknownRace {
    fn declare(&self, _: &mut Character) {}
    fn iterate(&self, _: &mut Character) {}
}

describe! { r#"
    # Unknown Race

    Please choose a race. This is a placeholder.
"#}