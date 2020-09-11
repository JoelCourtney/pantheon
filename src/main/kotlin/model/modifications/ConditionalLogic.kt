package model.modifications

import Engine
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.BooleanArrayDeserializer
import model.access.Expression
import model.gameObjects.Character
import model.modifications.effects.Effect

data class ConditionalLogic(
        @JsonProperty("if")
    @JsonDeserialize(using = BooleanArrayDeserializer::class)
    val ifTrue: Array<Expression<Boolean>> = arrayOf(),
        @JsonProperty("if not")
    @JsonDeserialize(using = BooleanArrayDeserializer::class)
    val ifFalse: Array<Expression<Boolean>> = arrayOf(),

        @JsonProperty("success results")
    val successModifications: Array<Modification> = arrayOf(),
        @JsonProperty("failure results")
    val failureModifications: Array<Modification> = arrayOf()

): Logic {
    override fun applyEffect(e: Engine) {
        TODO("Not yet implemented")
    }

    override fun applyResult(e: Engine): Effect? {
        TODO("Not yet implemented")
    }

    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }
}
