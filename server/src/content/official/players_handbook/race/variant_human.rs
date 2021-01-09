#[macros::race("Variant Human")]
pub struct VariantHuman {
    abilities: [Ability; 2],
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
    traits!([
        Trait {
            text: "# Ability Score Increase\n\nTwo different ability scores of your choice increase by 1.",
            #[choice(unique)]
            unique_choice: self.abilities
        },
        Trait {
            text: "# Skills\n\nYou gain proficiency in one skill of your choice.",
            choice: self.skill
        },
        Trait {
            text: "# Languages\n\nYou can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
            choice: self.language
        },
        Trait {
            text: "# Feat\n\nYou gain one feat of your choice.",
            choice: self.feat,
        }
    ]);

    feats!([
        self.feat
    ]);
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