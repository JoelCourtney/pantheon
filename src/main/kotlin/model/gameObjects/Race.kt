package model.gameObjects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.FileNameDeserializer.RaceFileNameDeserializer
import model.modifications.Modification
import model.modifications.effects.Effect

data class Race (
        @JsonProperty("name")
    val identity: String,
        val effects: Array<Effect>,
        val description: String,
        val selectable: Boolean = true,
        @JsonProperty("variant of")
    @JsonDeserialize(using = RaceFileNameDeserializer::class)
    val variantOf: Race? = null
): Prototype {
    override fun getEffects(data: List<String>): List<Effect> {
        TODO("Not yet implemented")
    }
}
