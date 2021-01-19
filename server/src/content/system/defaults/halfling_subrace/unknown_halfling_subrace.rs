#[macros::custom_content("HalflingSubrace")]
pub struct UnknownHalflingSubrace;

impl Content for UnknownHalflingSubrace {
    fn declare(&self, _: &mut Character) {}
    fn iterate(&self, _: &mut Character) {}
}

describe! { r#"
    # Unknown Halfling Subrace

    Please choose a halfling subrace. This is a placeholder.
"#}