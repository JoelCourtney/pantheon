package model.races

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.FileNameDeserializer.RaceFileNameDeserializer
import model.results.Result

open class Race (
    val name: String,
    val traits: Array<Result>,
    val description: String,
    open val chooseable: Boolean = true,
    @JsonProperty("variant of")
    @JsonDeserialize(using = RaceFileNameDeserializer::class)
    val variantOf: Race? = null
) {
    override fun toString(): String {
        return "Race(name='$name', traits=${traits.contentToString()}, description='$description', chooseable=$chooseable, variantOf=$variantOf)"
    }

}
