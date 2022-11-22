crate::name!("Dragonborn");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dragonborn {
    ancestry: DraconicAncestry
}

#[content]
impl Race for Dragonborn {
    fn resolve(&mut self, c: &mut Character) {
        i! {
            c.size = CreatureSize::Medium;
            c.speeds.walk = 30;
            c.languages >>= vec![Language::Common, Language::Draconic];
        }

        m! {
            c.abilities.strength += 2;
            c.abilities.charisma += 1;
        }

        i! {
            c.race_traits >>= vec! [
                Element::Str (
                    "**Ability Score Increase:** Your `Strength` score increases by 2, and your `Charisma` score increases by 1.",
                ),
                Element::Str (
                    "**Age:** Young dragonborn grow quickly. They walk hours after hatching, attain the size and development of a 10-year-old human child by the age of 3, and reach adulthood by 15. They live to be around 80.",
                ),
                Element::Str (
                    "**Alignment:** Dragonborn tend to extremes, making a conscious choice for one side or the other in the cosmic war between good and evil (represented by Bahamut and Tiamat, respectively). Most dragonborn are good, but those who side with Tiamat can be terrible villains.",
                ),
                Element::Str (
                    "**Size:** Dragonborn are taller and heavier than humans, standing well over 6 feet tall and averaging almost 250 pounds. Your size is Medium.",
                ),
                Element::Str (
                    "**Speed:** Your base walking speed is `30 feet`.",
                ),
                Element::Str (
                    "**Languages:** You can speak, read, and write `Common` and `Draconic`. Draconic is thought to be one of the oldest languages and is often used in the study of magic. The language sounds harsh to most other creatures and includes numerous hard consonants and sibilants.",
                ),
                Element::Choice {
                    text: indoc! { r#"
                        **Draconic Ancestry:** You have draconic ancestry. Choose one type of dragon from the Draconic Ancestry table. Your breath weapon and damage resistance are determined by the dragon type, as shown in the table.

                        Dragon | Damage Type | Breath Weapon
                        --- | --- | ---
                        Black | Acid | `5 by 30 ft.` line (Dex. save)
                        Blue | Lightning | `5 by 30 ft.` line (Dex. save)
                        Brass | Fire | `5 by 30 ft.` line (Dex. save)
                        Bronze | Lightning | `5 by 30 ft.` line (Dex. save)
                        Copper | Acid | `5 by 30 ft.` line (Dex. save)
                        Gold | Fire | `15 ft.` cone (Dex. save)
                        Green | Poison | `15 ft.` cone (Con. save)
                        Red | Fire | `15 ft.` cone (Dex. save)
                        Silver | Cold | `15 ft.` cone (Con. save)
                        White | Cold | `15 ft.` cone (Con. save)
                    "# },
                    data: &mut self.ancestry,
                    unique: false
                }
            ];
        }

        todo!();
    }

    description! {r#"
        # Dragonborn

        > Her father stood on the first of the three stairs that led down from the portal, unmoving. The scales of his face had grown paler around the edges, but Clanless Mehen still looked as if he could wrestle down a dire bear himself. His familiar well-worn armor was gone, replaced by violet-tinted scale armor with bright silvery tracings. There was a blazon on his arm as well, the mark of some foreign house. The sword at his back was the same, though, the one he had carried since even before he had found the twins left in swaddling at the gates of Arush Vayem.
        >
        > For all her life, Farideh had known that reading her father’s face was a skill she’d been fortunate to learn. A human who couldn’t spot the shift of her eyes or Havilar’s would certainly see only the indifference of a dragon in Clanless Mehen’s face. But the shift of scales, the arch of a ridge, the set of his eyes, the gape of his teeth—her father’s face spoke volumes.
        >
        > But every scale of it, this time, seemed completely still—the indifference of a dragon, even to Farideh.
        >
        > — Erin M. Evans, The Adversary

        Born of dragons, as their name proclaims, the dragonborn walk proudly through a world that greets them with fearful incomprehension. Shaped by draconic gods or the dragons themselves, dragonborn originally hatched from dragon eggs as a unique race, combining the best attributes of dragons and humanoids. Some dragonborn are faithful servants to true dragons, others form the ranks of soldiers in great wars, and still others find themselves adrift, with no clear calling in life.

        ## Proud Dragon Kin

        Dragonborn look very much like dragons standing erect in humanoid form, though they lack wings or a tail. The first dragonborn had scales of vibrant hues matching the colors of their dragon kin, but generations of interbreeding have created a more uniform appearance. Their small, fine scales are usually brass or bronze in color, sometimes ranging to scarlet, rust, gold, or copper-green. They are tall and strongly built, often standing close to 6½ feet tall and weighing 300 pounds or more. Their hands and feet are strong, talonlike claws with three fingers and a thumb on each hand.

        The blood of a particular type of dragon runs very strong through some dragonborn clans. These dragonborn often boast scales that more closely match those of their dragon ancestor—bright red, green, blue, or white, lustrous black, or gleaming metallic gold, silver, brass, copper, or bronze.

        ## Self-Sufficient Clans

        To any dragonborn, the clan is more important than life itself. Dragonborn owe their devotion and respect to their clan above all else, even the gods. Each dragonborn’s conduct reflects on the honor of his or her clan, and bringing dishonor to the clan can result in expulsion and exile. Each dragonborn knows his or her station and duties within the clan, and honor demands maintaining the bounds of that position.

        A continual drive for self-improvement reflects the self-sufficiency of the race as a whole. Dragonborn value skill and excellence in all endeavors. They hate to fail, and they push themselves to extreme efforts before they give up on something. A dragonborn holds mastery of a particular skill as a lifetime goal. Members of other races who share the same commitment find it easy to earn the respect of a dragonborn.

        Though all dragonborn strive to be self-sufficient, they recognize that help is sometimes needed in difficult situations. But the best source for such help is the clan, and when a clan needs help, it turns to another dragonborn clan before seeking aid from other races—or even from the gods.

        ## Dragonborn Names

        Dragonborn have personal names given at birth, but they put their clan names first as a mark of honor. A childhood name or nickname is often used among clutchmates as a descriptive term or a term of endearment. The name might recall an event or center on a habit.

        **Male Names:** Arjhan, Balasar, Bharash, Donaar, Ghesh, Heskan, Kriv, Medrash, Mehen, Nadarr, Pandjed, Patrin, Rhogar, Shamash, Shedinn, Tarhun, Torinn

        **Female Names:** Akra, Biri, Daar, Farideh, Harann, Havilar, Jheri, Kava, Korinn, Mishann, Nala, Perra, Raiann, Sora, Surina, Thava, Uadjit

        **Childhood Names:** Climber, Earbender, Leaper, Pious, Shieldbiter, Zealous

        **Clan Names:** Clethtinthiallor, Daardendrian, Delmirev, Drachedandion, Fenkenkabradon, Kepeshkmolik, Kerrhylon, Kimbatuul, Linxakasendalor, Myastan, Nemmonis, Norixius, Ophinshtalajiir, Prexijandilin, Shestendeliath, Turnuroth, Verthisathurgiesh, Yarjerit

        > DRACONIANS
        >
        > In the Dragonlance setting, the followers of the evil goddess Takhisis learned a dark ritual that let them corrupt the eggs of metallic dragons, producing evil dragonborn called draconians. Five types of draconians, corresponding to the five types of metallic dragons, fought for Takhisis in the War of the Lance: auraks (gold), baaz (brass), bozak (bronze), kapak (copper), and sivak (silver). In place of their draconic breath weapons, they have unique magical abilities.

        ## Dragonborn Traits

        Your draconic heritage manifests in a variety of traits you share with other dragonborn.

        ### Ability Score Increase

        Your `Strength` score increases by 2, and your `Charisma` score increases by 1.

        ### Age

        Young dragonborn grow quickly. They walk hours after hatching, attain the size and development of a 10-year-old human child by the age of 3, and reach adulthood by 15. They live to be around 80.

        ### Alignment

        Dragonborn tend to extremes, making a conscious choice for one side or the other in the cosmic war between good and evil (represented by Bahamut and Tiamat, respectively). Most dragonborn are good, but those who side with Tiamat can be terrible villains.

        ### Size

        Dragonborn are taller and heavier than humans, standing well over `6 feet` tall and averaging almost 250 pounds. Your size is `Medium`.

        ### Speed

        Your base walking speed is `30 feet`.

        ### Draconic Ancestry

        You have draconic ancestry. Choose one type of dragon from the Draconic Ancestry table. Your breath weapon and damage resistance are determined by the dragon type, as shown in the table.

        Dragon | Damage Type | Breath Weapon
        --- | --- | ---
        Black | Acid | `5 by 30 ft.` line (Dex. save)
        Blue | Lightning | `5 by 30 ft.` line (Dex. save)
        Brass | Fire | `5 by 30 ft.` line (Dex. save)
        Bronze | Lightning | `5 by 30 ft.` line (Dex. save)
        Copper | Acid | `5 by 30 ft.` line (Dex. save)
        Gold | Fire | `15 ft.` cone (Dex. save)
        Green | Poison | `15 ft.` cone (Con. save)
        Red | Fire | `15 ft.` cone (Dex. save)
        Silver | Cold | `15 ft.` cone (Con. save)
        White | Cold | `15 ft.` cone (Con. save)

        ### Breath Weapon

        You can use your action to exhale destructive energy. Your draconic ancestry determines the size, shape, and damage type of the exhalation. When you use your breath weapon, each creature in the area of the exhalation must make a saving throw, the type of which is determined by your draconic ancestry. The DC for this saving throw equals 8 + your Constitution modifier + your proficiency bonus. A creature takes 2d6 damage on a failed save, and half as much damage on a successful one. The damage increases to 3d6 at 6th level, 4d6 at 11th level, and 5d6 at 16th level. After you use your breath weapon, you can’t use it again until you complete a short or long rest.

        ### Damage Resistance

        You have resistance to the damage type associated with your draconic ancestry.

        ### Languages

        You can speak, read, and write `Common` and `Draconic`. `Draconic` is thought to be one of the oldest languages and is often used in the study of magic. The language sounds harsh to most other creatures and includes numerous hard consonants and sibilants.
    "#}
}

#[choose]
pub enum DraconicAncestry {
    Black,
    Blue,
    Brass,
    Bronze,
    Copper,
    Gold,
    Green,
    Red,
    Silver,
    White,
    Unknown
}