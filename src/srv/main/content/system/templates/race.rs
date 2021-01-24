// Includes boilerplate imports, and defines the NAME constant.
crate::name!("Template Race");

// These derives are required.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TemplateRace {
    // Add fields for data you want the user to be able to interact with.
    // For example, if this race lets the user choose an extra language proficiency:
    extra_language: Language
}

#[content]
impl Race for TemplateRace {
    properties! {}

    // Declare the fields in the character you intend to alter. See the documentation for Staged
    // for a description of iteration stages.
    fn declare(&self, c: &mut Character) {
        c.languages.declare_initializer(NAME);
    }

    // Check if it is time to make the declared changes, and then do them if so.
    fn iterate(&self, c: &mut Character) {
        if c.languages.initialize(NAME) {
            (*c.languages).push(self.extra_language);
        }
    }

    // This function only runs once; use it to push all of your race's traits to c.race_traits.
    // Features can include dropdown choices for the user to interact with.
    // See docs for Feature for more details.
    // You can in theory do some other cheaky stuff with this function, but I haven't found a use
    // for that yet.
    fn last(&mut self, c: &mut Character) {
        c.race_traits.extend(
            vec! [
                Feature (
                    "# Some Trait, idk\n\nIt be a trait.",
                    None
                )
                Feature (
                    "# Extra Language\n\n^^^ You get one of those.",
                    Some(&mut self.extra_language)
                )
            ]
        )
    }

    // FULL description of the race. I mean if this is from an official DnD book, this should be the
    // ENTIRE raw text, formatted nicely in markdown.
    description! {r#"
        # Template Race

        This is the general layout for race files.
    "#}
}