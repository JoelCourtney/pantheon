#[macros::custom_content("RoguishArchetype")]
pub struct UnknownRoguishArchetype;

impl LeveledContent for UnknownRoguishArchetype {
    fn declare(&self, _: &mut Character, _: usize) {}
    fn iterate(&self, _: &mut Character, _: usize) {}
}

describe! { r#"
    # Unknown Roguish Archetype

    Please choose a roguish archetype. This is a placeholder.
"#}