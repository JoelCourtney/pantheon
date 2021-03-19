crate::name!("Unknown");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownBackground;

#[content]
impl Background for UnknownBackground {
    properties! {}

    fn iterate(&self, c: &mut Character) {
        common_background_rules::iterate(c, self);
    }

    description! {r#"
        # Unknown Background

        Please choose a background. This is a placeholder.
    "#}
}