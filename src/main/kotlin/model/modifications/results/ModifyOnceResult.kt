package model.modifications.results

import Engine
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer
import model.gameObjects.Character
import model.access.Identifier
import model.modifications.effects.Effect

class ModifyOnceResult(
    @JsonProperty("modify once")
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
): Result {
    override fun applyResult(e: Engine): Effect? {
        TODO("Not yet implemented")
    }

    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }
}
