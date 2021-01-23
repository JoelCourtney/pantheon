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
        hit_dice: u32 = 6
    }
    fn declare(&self, c: &mut Character) {
        self.common.declare(c, self);
    }
    fn iterate(&self, c: &mut Character) {
        self.common.iterate(c, self);
    }

    description! { r#"
        # Wizard
    "#}
}

