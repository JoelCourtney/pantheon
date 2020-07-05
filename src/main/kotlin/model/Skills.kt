package model

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

interface Skill

enum class DexteritySkill(val symbol: String) : Skill {
    ACROBATICS("acrobatics"),
    SLEIGHT_OF_HAND("sleight of hand"),
    STEALTH("stealth");
}

enum class StrengthSkill(val symbol: String) : Skill {
    ATHLETICS("athletics");
}

enum class IntelligenceSkill(val symbol: String) : Skill {
    ARCANA("arcana"),
    HISTORY("history"),
    INVESTIGATION("investigation"),
    NATURE("nature"),
    RELIGION("religion");
}

enum class WisdomSkill(val symbol: String) : Skill {
    ANIMAL_HANDLING("animal handling"),
    INSIGHT("insight"),
    MEDICINE("medicine"),
    PERCEPTION("perception"),
    SURVIVAL("survival");
}

enum class CharismaSkill(val symbol: String) : Skill {
    DECEPTION("deception"),
    INTIMIDATION("intimidation"),
    PERFORMANCE("performance"),
    PERSUASION("persuasion");
}