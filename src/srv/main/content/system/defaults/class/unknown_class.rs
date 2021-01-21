#[macros::class("Unknown Class")]
pub struct UnknownClass;

#[typetag::serde]
impl Class for UnknownClass {
    properties! {
        hit_dice: usize = 0
    }
}

describe! { r#"
    # Unknown Class

    Please choose a class. This is a placeholder.
"#}