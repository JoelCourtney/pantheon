package model.gameObjects

import model.effects.Effect

class Feature(
        val name: String,
        val description: String,
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
