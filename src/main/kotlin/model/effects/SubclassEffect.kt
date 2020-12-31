package model.effects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.FileNameDeserializer
import model.gameObjects.prototypes.Subclass

data class SubclassEffect(
    @JsonProperty("subclass")
    val name: String
): Effect() {
    override fun apply() {
        TODO("Not yet implemented")
    }

    override fun findDependencies(): List<String> {
        TODO("Not yet implemented")
    }

    override fun findEffected(): List<String> {
        TODO("Not yet implemented")
    }
}