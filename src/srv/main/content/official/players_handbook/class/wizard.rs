// TODO

crate::name!("Wizard");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Wizard {
    common: CommonClassContent,
    spellbook: Vec<Box<dyn Spell>>
}

#[content]
impl Class for Wizard {
    properties! {
        hit_dice: usize = 6
    }
    fn declare(&self, _c: &mut Character) {

    }
}

describe! { r#"
    # Wizard
"#}