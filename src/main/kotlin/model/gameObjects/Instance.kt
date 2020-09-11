package model.gameObjects

import model.modifications.effects.Effect

data class Instance<T: Prototype>(
        val base: T,
        val choices: List<String>,
        val properties: Map<String,String>
) {
    fun getEffects(): List<Effect> {
        return base.getEffects(choices)
    }
}