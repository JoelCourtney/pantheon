package model.gameObjects.prototypes

import com.fasterxml.jackson.annotation.JsonProperty
import model.access.Identifier
import model.access.StringLiteral
import model.effects.ConditionalEffect
import model.effects.Effect

class Class(
    val name: String,
    val description: String,
    @JsonProperty("hit dice")
    val hitDice: Int,
    levels: List<List<Effect>>
): Prototype(standardEffects() + flattenLevelEffects(levels)) {
    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }

    companion object {
        fun standardEffects() = listOf<Effect>()

        fun flattenLevelEffects(levels: List<List<Effect>>): List<Effect> {
//            if (levels.size != 20) {
//                throw GameObjectParseException()
//            }
            return levels.mapIndexed { i, it -> ConditionalEffect(
                listOf(Identifier("root", listOf(StringLiteral("level ${i+1}+")))),
                null,
                it
            )
            }
        }
    }
}
