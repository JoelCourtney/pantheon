crate::name!("Unknown");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownClass;

#[content]
impl Class for UnknownClass {
    properties! {
        hit_dice: u32 = 0
    }

    description! { r#"
        # Unknown Class

        Please choose a class. This is a placeholder.
    "#}
}

