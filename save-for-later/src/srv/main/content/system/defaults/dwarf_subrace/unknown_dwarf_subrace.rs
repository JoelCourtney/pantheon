crate::name!("Unknown");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownDwarfSubrace;

#[content]
impl DwarfSubrace for UnknownDwarfSubrace {
    properties! {}

    description! { r#"
        # Unknown Dwarf Subrace

        Please choose a Dwarf subrace. This is a placeholder.
    "#}
}

