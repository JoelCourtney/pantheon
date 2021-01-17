#[macros::race("Variant Human")]
pub struct VariantHuman {
    abilities: [Ability; 2],
    skill: Skill,
    feat: Box<dyn Feat>,
    language: Language
}

impl Content for VariantHuman {
    fn initialize(&self, c: &mut Character) {
        c.size = CreatureSize::Medium;

        c.languages.push(Language::Common);
        c.languages.push(self.language);

        c.skill_proficiencies.push((self.skill, ProficiencyType::Single));
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
    fn write_features(&self) -> Vec<FeatureSerial> {
        vec! [
            FeatureSerial {
                text: "# Ability Score Increase\n\nTwo different ability scores of your choice increase by 1.",
                choose: self.abilities.to_choose_serial(true)
            },
            FeatureSerial {
                text: "# Skills\n\nYou gain proficiency in one skill of your choice.",
                choose: self.skill.to_choose_serial(false)
            },
            FeatureSerial {
                text: "# Languages\n\nYou can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                choose: self.language.to_choose_serial(false)
            },
            FeatureSerial {
                text: "# Feat\n\nYou gain one feat of your choice.",
                choose: self.feat.to_choose_serial(false)
            }
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
    fn write_feats(&self) -> Vec<Vec<FeatureSerial>> {
        vec![ self.feat.write_features() ]
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