package model.gameObjects

import model.modifications.effects.Effect

data class Class(
    val identity: String,
    val description: String,
    val effects: List<Effect>
): Prototype {
    override fun getEffects(data: List<String>): List<Effect> {
        TODO("Not yet implemented")
    }
}
