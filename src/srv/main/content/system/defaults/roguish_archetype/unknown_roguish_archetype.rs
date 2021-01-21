#[macros::custom_content("RoguishArchetype")]
pub struct UnknownRoguishArchetype;

#[typetag::serde]
impl RoguishArchetype for UnknownRoguishArchetype {
    properties! {}
}

describe! { r#"
    # Unknown Roguish Archetype

    Please choose a roguish archetype. This is a placeholder.
"#}