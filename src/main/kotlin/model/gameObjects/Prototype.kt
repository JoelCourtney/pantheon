package model.gameObjects

import model.modifications.Modification
import model.modifications.effects.Effect

interface Prototype {
    val name: String
    fun getEffects(data: List<String>): List<Effect>
}