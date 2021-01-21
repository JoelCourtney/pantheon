crate::name!("Unknown Halfling Subrace");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownHalflingSubrace;

#[content]
impl HalflingSubrace for UnknownHalflingSubrace {
    properties! {}
}

describe! { r#"
    # Unknown Halfling Subrace

    Please choose a halfling subrace. This is a placeholder.
"#}