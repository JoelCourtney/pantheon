package model.gameObjects

import model.modifications.Modification
import model.modifications.effects.Effect

data class Background(
        val name: String,
        val description: String,
        val effects: List<Effect>
): Prototype {
    override fun getResults(data: List<String>): List<Modification> {
        TODO("Not yet implemented")
    }
}