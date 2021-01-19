#[macros::feat("Unknown Feat")]
pub struct UnknownFeat;

impl Content for UnknownFeat {
    fn declare(&self, _: &mut Character) {}
    fn iterate(&self, _: &mut Character) {}
}

describe! {r#"
    # Unknown Feat

    Please choose a feat. This is a placeholder.
"#}