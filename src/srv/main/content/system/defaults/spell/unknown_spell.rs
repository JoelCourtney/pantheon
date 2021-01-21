#[macros::spell("Unknown Spell")]
pub struct UnknownSpell {}

#[typetag::serde]
impl Spell for UnknownSpell {
    properties! {}
}

describe! { r#"
    # Unknown Spell

    Please choose a spell. This is a placeholder.
"#}
