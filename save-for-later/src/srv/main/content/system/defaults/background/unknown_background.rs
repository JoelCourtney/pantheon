crate::name!("Unknown");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownBackground;

#[content]
impl Background for UnknownBackground {
    properties! {}

    description! {r#"
        # Unknown Background

        Please choose a background. This is a placeholder.
    "#}
}