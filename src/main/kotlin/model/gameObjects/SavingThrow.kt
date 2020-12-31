package model.gameObjects

import model.access.Evaluated
import model.access.Expression
import model.access.Identifier
import model.access.StringLiteral

enum class SavingThrow: Evaluated<SavingThrow> {
    STRENGTH,
    DEXTERITY,
    CONSTITUTION,
    INTELLIGENCE,
    WISDOM,
    CHARISMA,
    DEATH;

    companion object {
        private fun fromString(s: String): SavingThrow {
            return valueOf(s.toUpperCase())
        }

        fun fromStringExpression(e: Expression<String>): Expression<SavingThrow> {
            return when (e) {
                is StringLiteral -> fromString(e.string)
                is Identifier<String> -> e as Identifier<SavingThrow>
                else -> TODO("make an error for this")
            }
        }
    }
}