#[macros::race("Variant Human")]
pub struct VariantHuman {
    abilities: [Ability; 2],
    skill: Skill,
    feat: Box<dyn Feat>,
    language: Language
}

#[typetag::serde]
impl Race for VariantHuman {
    properties! {}

    fn declare(&self, c: &mut Character) {
        c.size.declare_initializer(NAME);
        c.languages.declare_initializer(NAME);
        c.skill_proficiencies.declare_initializer(NAME);

        self.feat.declare(c);
    }
    fn iterate(&self, c: &mut Character) {
        if c.size.initialize(NAME) {
            *c.size = CreatureSize::Medium;
        }

        if c.languages.initialize(NAME) {
            (*c.languages).push(Language::Common);
            (*c.languages).push(self.language);
        }

        if c.skill_proficiencies.initialize(NAME) {
            (*c.skill_proficiencies).push((self.skill, ProficiencyType::Single));
        }

        self.feat.iterate(c);
    }
    fn last(&mut self, c: &mut Character) {
        c.race_traits.extend(vec! [
            Feature (
                "# Ability Score Increase\n\nTwo different ability scores of your choice increase by 1.",
                Some(&mut self.abilities)
            ),
            Feature (
                "# Skills\n\nYou gain proficiency in one skill of your choice.",
                Some(&mut self.skill)
            ),
            Feature (
                "# Languages\n\nYou can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                Some(&mut self.language)
            ),
            Feature (
                "# Feat\n\nYou gain one feat of your choice.",
                Some(&mut self.feat)
            )
        ]);

        self.feat.last(c);
    }
}

describe! { r#"
    # Variant Human

    If your campaign uses the optional feat rules from the Player’s Handbook, your Dungeon Master might allow these variant traits, all of which replace the human’s Ability Score Increase trait.

    ## Variant Human Traits

    ### Ability Score Increase

    Two different ability scores of your choice increase by 1.

    ### Skills

    You gain proficiency in one skill of your choice.

    ### Feat

    You gain one feat of your choice.
"#}