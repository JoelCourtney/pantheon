package model.modifications.effects

import Engine
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer
import model.gameObjects.Character
import model.access.Identifier

data class SetEffect(
    @JsonProperty("set")
    @JsonDeserialize(using = IdentifierDeserializer::class)
    val id: Identifier<*>,
    val to: String
): Effect {
    override fun applyEffect(e: Engine) {
        TODO("Not yet implemented")
    }
}
