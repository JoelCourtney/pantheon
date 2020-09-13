package model.gameObjects

import model.access.Accessible
import model.access.Environment
import model.effects.Effect

data class Instance<T: Prototype>(
        val proto: T,
        val choices: List<String>
): Accessible {
    fun getEffects(): List<Effect> {
        val env = Environment(this)
        return proto.getEffects(env)
    }

    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }
}