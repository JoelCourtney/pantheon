package model.races

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.RaceDeserializer
import model.results.Result

@JsonDeserialize(using = RaceDeserializer::class)
open class Race (
    val name: String,
    val traits: Array<Result>,
    val description: String,
    open val chooseable: Boolean = true
) {
    protected constructor(r: Race): this(r.name, r.traits, r.description)
    override fun toString(): String {
        return "Race(name='$name', traits=${traits.contentToString()}, description='$description', chooseable=$chooseable)"
    }

}
