package model.gameObjects

import model.access.Evaluated

enum class Skill(val identity: String): Evaluated<Skill> {
    ACROBATICS("acrobatics"),
    ANIMAL_HANDLING("animal handling"),
    ARCANA("arcana"),
    ATHLETICS("athletics"),
    DECEPTION("deception"),
    HISTORY("history"),
    INSIGHT("insight"),
    INTIMIDATION("intimidation"),
    INVESTIGATION("investigation"),
    MEDICINE("medicine"),
    NATURE("nature"),
    PERCEPTION("perception"),
    PERFORMANCE("performance"),
    PERSUASION("persuasion"),
    RELIGION("religion"),
    SLEIGHT_OF_HAND("sleight of hand"),
    STEALTH("stealth"),
    SURVIVAL("survival");
    
    fun getAbility(): Ability {
        return when (this) {
            ATHLETICS -> Ability.STRENGTH
            ACROBATICS,
            SLEIGHT_OF_HAND,
            STEALTH -> Ability.DEXTERITY
            ARCANA,
            HISTORY,
            INVESTIGATION,
            NATURE,
            RELIGION -> Ability.INTELLIGENCE
            ANIMAL_HANDLING,
            INSIGHT,
            MEDICINE,
            PERCEPTION,
            SURVIVAL -> Ability.WISDOM
            DECEPTION,
            INTIMIDATION,
            PERFORMANCE,
            PERSUASION -> Ability.CHARISMA
        }
    }
}
