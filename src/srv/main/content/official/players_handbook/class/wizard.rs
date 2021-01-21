// TODO

crate::name!("Wizard");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Wizard {
    common: CommonClassContent

}

#[content]
impl Class for Wizard {
    properties! {
        hit_dice: usize = 6
    }
}

describe! { r#"
    # Wizard
"#}