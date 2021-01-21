#[macros::custom_content("HalflingSubrace")]
pub struct UnknownHalflingSubrace;

#[typetag::serde]
impl HalflingSubrace for UnknownHalflingSubrace {
    properties! {}
}

describe! { r#"
    # Unknown Halfling Subrace

    Please choose a halfling subrace. This is a placeholder.
"#}