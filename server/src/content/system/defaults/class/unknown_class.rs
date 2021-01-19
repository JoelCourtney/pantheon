#[macros::class("Unknown Class")]
pub struct UnknownClass;

impl Content for UnknownClass {
    fn declare(&self, _: &mut Character) {}
    fn modify(&self, _: &mut Character) {}
}

describe! { r#"
    # Unknown Class

    Please choose a class. This is a placeholder.
"#}