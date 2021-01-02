package model.gameObjects.prototypes

import exceptions.GameObjectParseException
import model.access.Identifier
import model.access.StringLiteral
import model.effects.ConditionalEffect
import model.effects.Effect

class Subclass(
    val name: String,
    val type: String,
    levels: List<List<Effect>>,
    val description: String
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
            if (levels.size != 20) {
                throw GameObjectParseException()
            }
            return levels.mapIndexed { i, it -> ConditionalEffect(
                listOf(Identifier("root", listOf(StringLiteral("level ${i+1}+")))),
                null,
                it
            )
            }
        }
    }
}
