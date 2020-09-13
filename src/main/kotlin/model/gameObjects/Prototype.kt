package model.gameObjects

import model.access.Accessible
import model.access.Environment
import model.effects.Effect

abstract class Prototype(
        val effects: List<Effect>
): Accessible {
    fun getEffects(env: Environment): List<Effect> {
        for (effect in effects) {
            effect.env = env
        }
        return effects
    }
}