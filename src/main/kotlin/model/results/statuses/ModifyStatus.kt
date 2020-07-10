package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer
import model.Character
import model.identity.Identifier

class ModifyStatus(
    @JsonProperty("modify")
    @JsonDeserialize(using = IdentifierDeserializer::class)
    val id: Identifier<*>,
    val add: String? = null,
    val subtract: String? = null,
    val multiply: String? = null,
    @JsonProperty("divide down")
    val divide: String? = null,
    @JsonProperty("divide up")
    val divideUp: String? = null,
    val append: Array<String> = arrayOf()
): Status() {
    override fun applyStatus(c: Character) {
        throw NotImplementedError()
    }
}
