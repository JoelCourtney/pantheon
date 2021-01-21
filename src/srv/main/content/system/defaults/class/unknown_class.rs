crate::name!("Unknown Class");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownClass;

#[content]
impl Class for UnknownClass {
    properties! {
        hit_dice: usize = 0
    }
}

describe! { r#"
    # Unknown Class

    Please choose a class. This is a placeholder.
"#}