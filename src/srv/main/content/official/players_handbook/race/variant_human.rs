crate::name!("Variant Human");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VariantHuman {
    abilities: [Ability; 2],
    skill: Skill,
    feat: Box<dyn Feat>,
    language: Language
}

#[content]
impl Race for VariantHuman {
    properties! {}

    fn iterate(&self, c: &mut Character) {
        common_race_rules::iterate(c, self);

        i! {
            c.size = CreatureSize::Medium;
            c.speeds.walk = 30;
        }

        if self.language != Language::Unknown {
            i! { c.languages >>= vec! [ Language::Common, self.language ] }
        } else {
            i! { c.languages <<= Language::Common }
        }

        for ability in &self.abilities {
            match c.abilities.get_mut(*ability) {
                Some(a) => {
                    m! { *a += 1 }
                }
                None => {}
            }
        }

        match c.skill_proficiencies.get_mut(self.skill) {
            Some(s) => {
                i! { *s = ProficiencyType::Single }
            }
            None => {}
        }

        self.feat.iterate(c);
    }
    fn last(&mut self, c: &mut Character) {
        c.race_traits.extend(vec! [
            Feature (
                "**Ability Score Increase:** Two different ability scores of your choice increase by 1.",
                Any(&mut self.abilities)
            ),
            Feature (
                "**Skills:** You gain proficiency in one skill of your choice.",
                Any(&mut self.skill)
            ),
            Feature (
                "**Languages:** You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                Any(&mut self.language)
            ),
            Feature (
                "**Feat:** You gain one feat of your choice.",
                Any(&mut self.feat)
            )
        ]);

        self.feat.last(c);
    }

    description! { r#"
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
}

