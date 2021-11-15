crate::name!("Unknown");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownFeat;

#[content]
impl Feat for UnknownFeat {
    properties! {}

    description! {r#"
        # Unknown Feat

        Please choose a feat. This is a placeholder.
    "#}
}

