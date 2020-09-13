package model.effects

import com.fasterxml.jackson.annotation.JsonProperty
import model.access.Accessible
import model.access.Environment
import model.access.Identifier

data class ChoiceEffect<T>(
    val choose: String,

    @JsonProperty("filter out")
    val filterOut: Identifier<Array<String>>? = null,

    @JsonProperty("filter except")
    val filterExcept: Identifier<Array<String>>? = null,
    
    @JsonProperty("filter out list")
    val filterOutList: Array<String> = arrayOf(),
    
    @JsonProperty("filter except list")
    val filterExceptList: Array<String> = arrayOf(),

    @JsonProperty("choices")
    val numChoices: Int = 1,
    val unique: Boolean = false,

    val effects: Array<Effect>
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
        return listOfNotNull(
                filterOut?.characterAttribute,
                filterExcept?.characterAttribute
        ) + effects.flatMap { it.dependencies!! }
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
