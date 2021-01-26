// TODO

crate::name!("Wizard");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Wizard {
    spellbook: Vec<Box<dyn Spell>>
}

#[content]
impl Class for Wizard {
    properties! {
        hit_dice: u32 = 6
    }
    fn declare(&self, c: &mut Character, level: u32, first: bool) {
        common_class_rules::declare(self, c, level, first);
    }
    fn iterate(&self, c: &mut Character, level: u32, first: bool) {
        common_class_rules::iterate(self, c, level, first);
    }

    fn last(&mut self, c: &mut Character, _level: u32, _first: bool) {
        c.class_features.push(
            Feature (
                "Hello",
                Unique(&mut self.spellbook)
            )
        )
    }

    description! { r#"
        # Wizard
    "#}
}

