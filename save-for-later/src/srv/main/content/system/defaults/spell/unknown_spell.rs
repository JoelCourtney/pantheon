crate::name!("Unknown");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownSpell {}

#[content]
impl Spell for UnknownSpell {
    properties! {
        level: usize = 0,
        casting_time: CastingTime = CastingTime::Ritual("this is not a real spell"),
        optional_ritual: bool = false
    }

    description! { r#"
        # Unknown Spell

        Please choose a spell. This is a placeholder.
    "#}
}

