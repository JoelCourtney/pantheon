#[macros::race("Variant Human")]
pub struct VariantHuman {
    abilities: [Ability; 2],
    skill: Skill,
    feat: Box<dyn Feat>,
    language: Language
}

impl Content for VariantHuman {
    fn declare(&self, c: &mut Character) {
        c.size.declare_initializer(NAME);
        c.languages.declare_initializer(NAME);
        c.skill_proficiencies.declare_initializer(NAME);
    }
    fn modify(&self, c: &mut Character) {
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
    }

    fn receive_choice(&mut self, choice: &str, feature_index: usize, choice_index: usize) {
        match feature_index {
            0 => self.abilities.choose(choice, choice_index),
            1 => self.skill.choose(choice, choice_index),
            2 => self.language.choose(choice, choice_index),
            3 => self.feat.choose(choice, choice_index),
            _ => unimplemented!()
        }
    }
    fn features(&self) -> Vec<Feature> {
        vec! [
            Feature (
                "# Ability Score Increase\n\nTwo different ability scores of your choice increase by 1.",
                Some(self.abilities.to_choice(true))
            ),
            Feature (
                "# Skills\n\nYou gain proficiency in one skill of your choice.",
                Some(self.skill.to_choice(false))
            ),
            Feature (
                "# Languages\n\nYou can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                Some(self.language.to_choice(false))
            ),
            Feature (
                "# Feat\n\nYou gain one feat of your choice.",
                Some(self.feat.to_choice(false))
            )
        ]
    }

    fn receive_feat_choice(&mut self, choice: &str, feat_index: usize, feature_index: usize, choice_index: usize) -> Result<(),()> {
        match feat_index {
            0 => {
                self.feat.receive_choice(choice, feature_index, choice_index);
                Ok(())
            }
            _ => Err(())
        }
    }
    fn feats(&self) -> Vec<Vec<Feature>> {
        vec![ self.feat.features() ]
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