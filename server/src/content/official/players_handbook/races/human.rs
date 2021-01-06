macros::race!("Human");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Human {
    pub extra_language: Language
}

impl Featured for Human {
    fn features(&mut self) -> Vec<Trait> {
        vec![
            Trait {
                name: "Age",
                description: "Humans reach adulthood in their late teens and live less than a century.",
                ..Default::default()
            },
            Trait {
                name: "Languages",
                description: "You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                choice: Choice::Language(&mut self.extra_language)
            }
        ]
    }
}

impl Modify for Human {
    fn initialize(&self, c: &mut Character) {
        c.size = CreatureSize::Medium;
        c.walking_speed = 30;

        c.proficiencies.push(Proficiency::Language(Language::Common));
        c.proficiencies.push(Proficiency::Language(self.extra_language.clone()));
    }
    fn modify(&self, c: &mut Character) {
        c.strength += 1;
        c.dexterity += 1;
        c.constitution += 1;
        c.intelligence += 1;
        c.wisdom += 1;
        c.charisma += 1;
    }
}
