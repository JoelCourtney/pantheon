crate::name!("Unknown Spell");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownSpell {}

#[content]
impl Spell for UnknownSpell {
    properties! {}
}

describe! { r#"
    # Unknown Spell

    Please choose a spell. This is a placeholder.
"#}
