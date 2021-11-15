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
    fn resolve(&mut self, c: &mut Character) {
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

        i! {
            c.race_traits >>= vec! [
                Element::Choice {
                    text: "**Ability Score Increase:** Two different ability scores of your choice increase by 1.",
                    data: &mut self.abilities,
                    unique: false
                },
                Element::Choice {
                    text: "**Skills:** You gain proficiency in one skill of your choice.",
                    data: &mut self.skill,
                    unique: false
                },
                Element::Choice {
                    text: "**Languages:** You can speak, read, and write `Common` and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                    data: &mut self.language,
                    unique: false
                },
                Element::Choice {
                    text: "**Feat:** You gain one feat of your choice.",
                    data: &mut self.feat,
                    unique: false
                }
            ];
        }

        self.feat.resolve(c);
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

