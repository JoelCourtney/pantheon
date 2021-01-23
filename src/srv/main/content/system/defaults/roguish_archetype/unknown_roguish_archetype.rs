crate::name!("Unknown Roguish Archetype");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownRoguishArchetype;

#[content]
impl RoguishArchetype for UnknownRoguishArchetype {
    properties! {}

    description! { r#"
        # Unknown Roguish Archetype

        Please choose a roguish archetype. This is a placeholder.
    "#}
}

