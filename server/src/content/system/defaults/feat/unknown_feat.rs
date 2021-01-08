#[macros::feat("Unknown Feat")]
pub struct UnknownFeat;

impl Modify for UnknownFeat {}
impl Featured for UnknownFeat {}

describe! {r#"
    # Unknown Feat

    Please choose a feat. This is a placeholder.
"#}