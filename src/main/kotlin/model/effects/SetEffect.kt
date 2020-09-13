package model.effects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer
import model.access.Environment
import model.access.Identifier

data class SetEffect(
    @JsonProperty("set")
    @JsonDeserialize(using = IdentifierDeserializer::class)
    val id: Identifier<*>,
    val to: String
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
