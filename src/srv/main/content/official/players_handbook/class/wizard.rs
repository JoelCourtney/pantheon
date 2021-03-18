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
    fn iterate(&self, c: &mut Character, level: u32, first: bool) {
        common_class_rules::iterate(self, c, level, first);
    }

    fn last(&mut self, c: &mut Character, level: u32, first: bool) {
        if let Some(s) = self.spellbook.last() {
            if s.name() != "Unknown Spell" {
                self.spellbook.push(Default::default())
            }
        }

        common_class_rules::last(self, c, level, first);

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

