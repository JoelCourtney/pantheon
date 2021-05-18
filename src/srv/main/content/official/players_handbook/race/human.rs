crate::name!("Human");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Human {
    extra_language: Language
}

#[content]
impl Race for Human {
    fn iterate(&self, c: &mut Character) {
        common_race_rules::iterate(c, self);

        i! {
            c.size = CreatureSize::Medium;
            c.speeds.walk = 30;
        }

        if self.extra_language != Language::Unknown {
            i! { c.languages >>= vec! [ Language::Common, self.extra_language ] }
        } else {
            i! { c.languages <<= Language::Common }
        }

        // MODIFY
        for ability in Ability::into_enum_iter() {
            if ability.known() {
                m! { c.abilities.get_mut_known(ability) += 1 }
            }
        }
    }
    fn last(&mut self, c: &mut Character) {
        c.race_traits.extend(vec! [
            Feature (
                "**Ability Score Increase:** Your ability scores each increase by `1`.",
                Empty
            ),
            Feature (
                "**Alignment:** Humans tend toward no particular alignment. The best and the worst are found among them.",
                Empty
            ),
            Feature (
                "**Age:** Humans reach adulthood in their late teens and live less than a century.",
                Empty
            ),
            Feature (
                "**Size:** Humans vary widely in height and build, from barely `5 feet` to well over `6 feet` tall. Regardless of your position in that range, your size is `Medium`.",
                Empty
            ),
            Feature (
                "**Speed:** Your base walking speed is `30 feet`.",
                Empty
            ),
            Feature (
                "**Languages:** You can speak, read, and write `Common` and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.",
                Any(&mut self.extra_language)
            )
        ]);
    }

    description! { r#"
        # Human

        > These were the stories of a restless people who long ago took to the seas and rivers in longboats, first to pillage and terrorize, then to settle. Yet there was an energy, a love of adventure, that sang from every page. Long into the night Liriel read, lighting candle after precious candle.
        >
        > She’d never given much thought to humans, but these stories fascinated her. In these yellowed pages were tales of bold heroes, strange and fierce animals, mighty primitive gods, and a magic that was part and fabric of that distant land.
        >
        > — *Elaine Cunningham, Daughter of the Drow*

        In the reckonings of most worlds, humans are the youngest of the common races, late to arrive on the world scene and short-lived in comparison to dwarves, elves, and dragons. Perhaps it is because of their shorter lives that they strive to achieve as much as they can in the years they are given. Or maybe they feel they have something to prove to the elder races, and that’s why they build their mighty empires on the foundation of conquest and trade. Whatever drives them, humans are the innovators, the achievers, and the pioneers of the worlds.

        ## A Broad Spectrum

        With their penchant for migration and conquest, humans are more physically diverse than other common races. There is no typical human. An individual can stand from 5 feet to a little over 6 feet tall and weigh from 125 to 250 pounds. Human skin shades range from nearly black to very pale, and hair colors from black to blond (curly, kinky, or straight); males might sport facial hair that is sparse or thick. A lot of humans have a dash of nonhuman blood, revealing hints of elf, orc, or other lineages. Humans reach adulthood in their late teens and rarely live even a single century.

        ## Variety in All Things

        Humans are the most adaptable and ambitious people among the common races. They have widely varying tastes, morals, and customs in the many different lands where they have settled. When they settle, though, they stay: they build cities to last for the ages, and great kingdoms that can persist for long centuries. An individual human might have a relatively short life span, but a human nation or culture preserves traditions with origins far beyond the reach of any single human’s memory. They live fully in the present—making them well suited to the adventuring life—but also plan for the future, striving to leave a lasting legacy. Individually and as a group, humans are adaptable opportunists, and they stay alert to changing political and social dynamics.

        > **EVERYONE'S SECOND-BEST FRIENDS**
        >
        > Just as readily as they mix with each other, humans mingle with members of other races. They get along with almost everyone, though they might not be close to many. Humans serve as ambassadors, diplomats, magistrates, merchants, and functionaries of all kinds.
        >
        > *Dwarves.* "They’re stout folk, stalwart friends, and true to their word. Their greed for gold is their downfall, though."
        >
        > *Elves.* "It’s best not to wander into elven woods. They don’t like intruders, and you’ll as likely be bewitched as peppered with arrows. Still, if an elf can get past that damned racial pride and actually treat you like an equal, you can learn a lot from them."
        >
        > *Halflings.* "It’s hard to beat a meal in a halfling home, as long as you don’t crack your head on the ceiling—good food and good stories in front of a nice, warm fire. If halflings had a shred of ambition, they might really amount to something."

        ## Lasting Institutions

        Where a single elf or dwarf might take on the responsibility of guarding a special location or a powerful secret, humans found sacred orders and institutions for such purposes. While dwarf clans and halfling elders pass on the ancient traditions to each new generation, human temples, governments, libraries, and codes of law fix their traditions in the bedrock of history. Humans dream of immortality, but (except for those few who seek undeath or divine ascension to escape death’s clutches) they achieve it by ensuring that they will be remembered when they are gone.

        Although some humans can be xenophobic, in general their societies are inclusive. Human lands welcome large numbers of nonhumans compared to the proportion of humans who live in nonhuman lands.

        ## Exemplars of Ambition

        Humans who seek adventure are the most daring and ambitious members of a daring and ambitious race. They seek to earn glory in the eyes of their fellows by amassing power, wealth, and fame. More than other people, humans champion causes rather than territories or groups.

        ## Human Names and Ethnicities

        Having so much more variety than other cultures, humans as a whole have no typical names. Some human parents give their children names from other languages, such as Dwarvish or Elvish (pronounced more or less correctly), but most parents give names that are linked to their region’s culture or to the naming traditions of their ancestors.

        The material culture and physical characteristics of humans can change wildly from region to region. In the Forgotten Realms, for example, the clothing, architecture, cuisine, music, and literature are different in the northwestern lands of the Silver Marches than in distant Turmish or Impiltur to the east—and even more distinctive in far-off Kara-Tur. Human physical characteristics, though, vary according to the ancient migrations of the earliest humans, so that the humans of the Silver Marches have every possible variation of coloration and features.

        In the Forgotten Realms, nine human ethnic groups are widely recognized, though over a dozen others are found in more localized areas of Faerûn. These groups, and the typical names of their members, can be used as inspiration no matter which world your human is in.

        ### Calishite

        Shorter and slighter in build than most other humans, Calishites have dusky brown skin, hair, and eyes. They’re found primarily in southwest Faerûn.

        **Calishite Names:** (Male) Aseir, Bardeid, Haseid, Khemed, Mehmen, Sudeiman, Zasheir; (female) Atala, Ceidil, Hama, Jasmal, Meilil, Seipora, Yasheira, Zasheida; (surnames) Basha, Dumein, Jassan, Khalid, Mostana, Pashar, Rein

        ### Chondathan

        Chondathans are slender, tawny-skinned folk with brown hair that ranges from almost blond to almost black. Most are tall and have green or brown eyes, but these traits are hardly universal. Humans of Chondathan descent dominate the central lands of Faerûn, around the Inner Sea.

        **Chondathan Names:** (Male) Darvin, Dorn, Evendur, Gorstag, Grim, Helm, Malark, Morn, Randal, Stedd; (female) Arveene, Esvele, Jhessail, Kerri, Lureene, Miri, Rowan, Shandri, Tessele; (surnames) Amblecrown, Buckman, Dundragon, Evenwood, Greycastle, Tallstag

        ### Damaran

        Found primarily in the northwest of Faerûn, Damarans are of moderate height and build, with skin hues ranging from tawny to fair. Their hair is usually brown or black, and their eye color varies widely, though brown is most common.

        **Damaran Names:** (Male) Bor, Fodel, Glar, Grigor, Igan, Ivor, Kosef, Mival, Orel, Pavel, Sergor; (female) Alethra, Kara, Katernin, Mara, Natali, Olma, Tana, Zora; (surnames) Bersk, Chernin, Dotsk, Kulenov, Marsk, Nemetsk, Shemov, Starag

        ### Illuskan

        Illuskans are tall, fair-skinned folk with blue or steely gray eyes. Most have raven-black hair, but those who inhabit the extreme northwest have blond, red, or light brown hair.

        **Illuskan Names:** (Male) Ander, Blath, Bran, Frath, Geth, Lander, Luth, Malcer, Stor, Taman, Urth; (female) Amafrey, Betha, Cefrey, Kethra, Mara, Olga, Silifrey, Westra; (surnames) Brightwood, Helder, Hornraven, Lackman, Stormwind, Windrivver

        ### Mulan

        Dominant in the eastern and southeastern shores of the Inner Sea, the Mulan are generally tall, slim, and amber-skinned, with eyes of hazel or brown. Their hair ranges from black to dark brown, but in the lands where the Mulan are most prominent, nobles and many other Mulan shave off all their hair.

        **Mulan Names:** (Male) Aoth, Bareris, Ehput-Ki, Kethoth, Mumed, Ramas, So-Kehur, Thazar-De, Urhur; (female) Arizima, Chathi, Nephis, Nulara, Murithi, Sefris, Thola, Umara, Zolis; (surnames) Ankhalab, Anskuld, Fezim, Hahpet, Nathandem, Sepret, Uuthrakt

        ### Rashemi

        Most often found east of the Inner Sea and often intermingled with the Mulan, Rashemis tend to be short, stout, and muscular. They usually have dusky skin, dark eyes, and thick black hair.

        **Rashemi Names:** (Male) Borivik, Faurgar, Jandar, Kanithar, Madislak, Ralmevik, Shaumar, Vladislak; (female) Fyevarra, Hulmarra, Immith, Imzel, Navarra, Shevarra, Tammith, Yuldra; (surnames) Chergoba, Dyernina, Iltazyara, Murnyethara, Stayanoga, Ulmokina

        ### Shou

        The Shou are the most numerous and powerful ethnic group in Kara-Tur, far to the east of Faerûn. They are yellowish-bronze in hue, with black hair and dark eyes. Shou surnames are usually presented before the given name.

        **Shou Names:** (Male) An, Chen, Chi, Fai, Jiang, Jun, Lian, Long, Meng, On, Shan, Shui, Wen; (female) Bai, Chao, Jia, Lei, Mei, Qiao, Shui, Tai; (surnames) Chien, Huang, Kao, Kung, Lao, Ling, Mei, Pin, Shin, Sum, Tan, Wan

        ### Tethyrian

        Widespread along the entire Sword Coast at the western edge of Faerûn, Tethyrians are of medium build and height, with dusky skin that tends to grow fairer the farther north they dwell. Their hair and eye color varies widely, but brown hair and blue eyes are the most common. Tethyrians primarily use Chondathan names.

        ### Turami

        Native to the southern shore of the Inner Sea, the Turami people are generally tall and muscular, with dark mahogany skin, curly black hair, and dark eyes.

        **Turami Names:** (Male) Anton, Diero, Marcon, Pieron, Rimardo, Romero, Salazar, Umbero; (female) Balama, Dona, Faila, Jalana, Luisa, Marta, Quara, Selise, Vonda; (surnames) Agosto, Astorio, Calabra, Domine, Falone, Marivaldi, Pisacar, Ramondo

        ## Human Traits

        It’s hard to make generalizations about humans, but your human character has these traits.

        ### Ability Score Increase

        Your ability scores each increase by 1.

        ### Age

        Humans reach adulthood in their late teens and live less than a century.

        ### Alignment

        Humans tend toward no particular alignment. The best and the worst are found among them.

        ### Size

        Humans vary widely in height and build, from barely 5 feet to well over 6 feet tall. Regardless of your position in that range, your size is Medium.

        ### Speed

        Your base walking speed is 30 feet.

        ### Languages

        You can speak, read, and write Common and one extra language of your choice. Humans typically learn the languages of other peoples they deal with, including obscure dialects. They are fond of sprinkling their speech with words borrowed from other tongues: Orc curses, Elvish musical expressions, Dwarvish military phrases, and so on.
    "# }
}

