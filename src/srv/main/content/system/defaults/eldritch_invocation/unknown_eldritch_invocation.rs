crate::name!("Unknown Eldritch Invocation");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownEldritchInvocation;

#[content]
impl EldritchInvocation for UnknownEldritchInvocation {
    properties! {}

    description! { r#"
        # Unknown Eldritch Invocation

        Please choose an Eldritch Invocation. This is a placeholder.
    "#}
}

