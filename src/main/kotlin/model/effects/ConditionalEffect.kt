package model.effects

import com.fasterxml.jackson.annotation.JsonProperty
import model.access.Accessible
import model.access.Environment
import model.access.Identifier

data class ConditionalEffect(
        @JsonProperty("if")
        val ifCond: List<Identifier<*>>?,
        @JsonProperty("if not")
        val ifNotCond: List<Identifier<*>>?,
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
        return (ifCond?.mapNotNull{ it.characterAttribute } ?: listOf()) +
                (ifNotCond?.mapNotNull { it.characterAttribute } ?: listOf()) +
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
