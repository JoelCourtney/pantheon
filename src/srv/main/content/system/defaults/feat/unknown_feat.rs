#[macros::feat("Unknown Feat")]
pub struct UnknownFeat;

#[typetag::serde]
impl Feat for UnknownFeat {
    properties! {}
}

describe! {r#"
    # Unknown Feat

    Please choose a feat. This is a placeholder.
"#}