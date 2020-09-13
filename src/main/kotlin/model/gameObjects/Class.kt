package model.gameObjects

import model.access.Environment
import model.effects.Effect

class Class(
    val name: String,
    val description: String,
    val hitDice: Int,
    effects: List<Effect>
): Prototype(standardEffects() + effects) {
    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }

    companion object {
        fun standardEffects() = listOf<Effect>()
    }
}
