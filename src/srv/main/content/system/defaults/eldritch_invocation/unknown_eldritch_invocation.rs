#[macros::custom_content("EldritchInvocation")]
pub struct UnknownEldritchInvocation;

#[typetag::serde]
impl EldritchInvocation for UnknownEldritchInvocation {
    properties! {}
}

describe! { r#"
    # Unknown Eldritch Invocation

    Please choose an Eldritch Invocation. This is a placeholder.
"#}