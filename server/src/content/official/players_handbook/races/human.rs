macros::race!("Human");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Human {
    pub extra_language: Language
}

impl Modify for Human {
    fn initialize(&self, c: &mut Character) {
        c.size = CreatureSize::Medium;
        c.walking_speed = 30;

        c.languages.push(Language::Common);
        c.languages.push(self.extra_language.clone());
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

impl Featured for Human {
    fn features(&mut self) -> Vec<Trait> {
        vec![
            Trait {
                name: "Ability Score Increase",
                description: "Your ability scores each increase by 1.",
                ..def!()
            },
            Trait {
                name: "Age",
                description: "Humans reach adulthood in their late teens and live less than a century.",
                ..def!()
            },
            Trait {
                name: "Alignment",
                description: "Humans tend toward no particular alignment. The best and the worst are found among them.",
                ..def!()
            },
            Trait {
                name: "Size",
                description: "Humans vary widely in height and build, from barely 5 feet to well over 6 feet tall. Regardless of your position in that range, your size is Medium.",
                ..def!()
            },
            Trait {
                name: "Speed",
                description: "Your base walking speed is 30 feet.",
                ..def!()
            },
            Trait {
                name: "Languages",
                description: "You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                choice: Choice::Language(&mut self.extra_language)
            }
        ]
    }
}
