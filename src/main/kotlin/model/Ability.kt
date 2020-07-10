package model

import model.identity.Evaluated

enum class Ability(val identity: String): Evaluated<Ability> {
    STRENGTH("strength"),
    DEXTERITY("dexterity"),
    CONSTITUTION("constitution"),
    INTELLIGENCE("intelligence"),
    WISDOM("wisdom"),
    CHARISMA("charisma");

    override fun toString(): String {
        return this.name.toLowerCase()
    }

    companion object {
        fun fromString(s: String): Ability {
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
