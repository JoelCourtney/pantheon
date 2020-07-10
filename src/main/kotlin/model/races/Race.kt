package model.races

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.FileNameDeserializer.RaceFileNameDeserializer
import model.identity.Evaluated
import model.results.Result

open class Race (
    @JsonProperty("name")
    val identity: String,
    val results: Array<Result>,
    val description: String,
    open val chooseable: Boolean = true,
    @JsonProperty("variant of")
    @JsonDeserialize(using = RaceFileNameDeserializer::class)
    val variantOf: Race? = null
): Evaluated<Race> {
    override fun toString(): String {
        return "Race(name='$identity', traits=${results.contentToString()}, description='$description', chooseable=$chooseable, variantOf=$variantOf)"
    }

}
