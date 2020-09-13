package model.gameObjects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.FileNameDeserializer.RaceFileNameDeserializer
import model.access.Environment
import model.effects.Effect

class Race (
        val name: String,
        val description: String,
        val selectable: Boolean = true,
        @JsonProperty("variant of")
        @JsonDeserialize(using = RaceFileNameDeserializer::class)
        val variantOf: Race? = null,
        effects: List<Effect>
): Prototype(standardEffects() + effects) {
    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }

    companion object {
        fun standardEffects() = listOf<Effect>()
    }
}
