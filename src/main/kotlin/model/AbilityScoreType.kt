package model

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.AbilityScoreTypeDeserializer

@JsonDeserialize(using = AbilityScoreTypeDeserializer::class)
enum class AbilityScoreType {
    STRENGTH,
    DEXTERITY,
    CONSTITUTION,
    INTELLIGENCE,
    WISDOM,
    CHARISMA;

    override fun toString(): String {
        return this.name.toLowerCase()
    }

    companion object {
        fun fromString(s: String): AbilityScoreType {
            return when (s.toLowerCase()) {
                "str", "strength" -> STRENGTH
                "dex", "dexterity" -> DEXTERITY
                "con", "constitution" -> CONSTITUTION
                "int", "intelligence" -> INTELLIGENCE
                "wis", "wisdom" -> WISDOM
                "cha", "charisma" -> CHARISMA
                else -> throw IllegalArgumentException("Ability score type not recognized.")
            }
        }
    }
}