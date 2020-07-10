package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer
import model.Character
import model.identity.Identifier

data class SetStatus(
    @JsonProperty("set")
    @JsonDeserialize(using = IdentifierDeserializer::class)
    val id: Identifier<*>,
    val to: String
): Status() {
    override fun applyStatus(c: Character) {
        id.set(to)
    }
}
