package model.gameObjects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.FileNameDeserializer.RaceFileNameDeserializer
import model.access.Evaluated
import model.results.Result

data class Race (
    @JsonProperty("name")
    val identity: String,
    val results: Array<Result>,
    val description: String,
    val selectable: Boolean = true,
    @JsonProperty("variant of")
    @JsonDeserialize(using = RaceFileNameDeserializer::class)
    val variantOf: Race? = null
): Evaluated<Race> {
    override fun toString(): String {
        return "Race(name='$identity', traits=${results.contentToString()}, description='$description', selectable=$selectable, variantOf=$variantOf)"
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Race

        if (identity != other.identity) return false
        if (!results.contentEquals(other.results)) return false
        if (description != other.description) return false
        if (selectable != other.selectable) return false
        if (variantOf != other.variantOf) return false

        return true
    }

    override fun hashCode(): Int {
        var result = identity.hashCode()
        result = 31 * result + results.contentHashCode()
        result = 31 * result + description.hashCode()
        result = 31 * result + selectable.hashCode()
        result = 31 * result + (variantOf?.hashCode() ?: 0)
        return result
    }

}
