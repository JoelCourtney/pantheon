#[macros::class("Unknown Class")]
pub struct UnknownClass;

impl Modify for UnknownClass {}
impl Featured for UnknownClass {}

describe! { r#"
    # Unknown Class

    Please choose a class. This is a placeholder.
"#}