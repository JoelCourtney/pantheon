#[macros::feat("Unknown Feat")]
pub struct UnknownFeat;

impl Content for UnknownFeat {}

describe! {r#"
    # Unknown Feat

    Please choose a feat. This is a placeholder.
"#}