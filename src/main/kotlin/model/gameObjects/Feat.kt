package model.gameObjects

import model.modifications.Modification
import model.modifications.effects.Effect

data class Feat(
    val name: String,
    val description: String,
    val effects: Array<Effect>
): Prototype {
    override fun getEffects(data: List<String>): List<Effect> {
        TODO("Not yet implemented")
    }
}
