#[macros::race("Variant Human")]
pub struct VariantHuman {
    ability1: Ability,
    ability2: Ability,
    skill: Skill,
    feat: Box<dyn Feat>,
    language: Language
}

impl Modify for VariantHuman {
    fn initialize(&self, c: &mut Character) {
        c.size = CreatureSize::Medium;

        c.languages.push(Language::Common);
        c.languages.push(self.language);

        c.skill_proficiencies.push((self.skill, ProficiencyType::Single));
    }
}

impl Featured for VariantHuman {
    fn features(&mut self) -> Vec<Trait> {
        vec![
            Trait {
                name: "Ability Score Increase",
                description: "Two different ability scores of your choice increase by 1.",
                choose: Ability::choose_multiple(vec![ &mut self.ability1, &mut self.ability2 ])
            },
            Trait {
                name: "Skills",
                description: "You gain proficiency in one skill of your choice.",
                choose: Skill::choose(&mut self.skill)
            },
            Trait {
                name: "Languages",
                description: "You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                choose: Language::choose(&mut self.language)
            },
            Trait {
                name: "Feat",
                description: "You gain one Feat of your choice.",
                choose: Box::choose(&mut self.feat)
            }
        ]
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