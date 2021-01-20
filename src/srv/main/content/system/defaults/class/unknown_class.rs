#[macros::class("Unknown Class")]
pub struct UnknownClass;

impl Content for UnknownClass {}

describe! { r#"
    # Unknown Class

    Please choose a class. This is a placeholder.
"#}