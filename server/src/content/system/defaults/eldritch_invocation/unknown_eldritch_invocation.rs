#[macros::custom_content("EldritchInvocation")]
pub struct UnknownEldritchInvocation;

impl Content for UnknownEldritchInvocation {
    fn declare(&self, _: &mut Character) {}
    fn iterate(&self, _: &mut Character) {}
}

describe! { r#"
    # Unknown Eldritch Invocation

    Please choose an Eldritch Invocation. This is a placeholder.
"#}