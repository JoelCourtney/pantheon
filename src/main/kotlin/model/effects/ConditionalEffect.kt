package model.effects

import model.access.Accessible
import model.access.Environment
import model.access.Identifier

data class ConditionalEffect(
        val ifCond: List<Identifier<Boolean>>,
        val ifFalseCond: List<Identifier<Boolean>>,
        val effects: List<Effect>
): Effect(), Accessible {
    override var env: Environment?
        get() = super.env
        set(value) {
            super.env = value
            val newEnv = value!!.nest(this)
            for (effect in effects) {
                effect.env = newEnv
            }
        }

    override fun apply() {
        TODO("Not yet implemented")
    }

    override fun findDependencies(): List<String> {
        return ifCond.mapNotNull{ it.characterAttribute } +
                ifFalseCond.mapNotNull { it.characterAttribute } +
                effects.flatMap { it.dependencies!! }
    }

    override fun findEffected(): List<String> {
        return effects.flatMap { it.effected!! }
    }

    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }
}
