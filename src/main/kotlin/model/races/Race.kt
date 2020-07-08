package model.races

import com.fasterxml.jackson.annotation.JsonProperty
import model.results.Result

open class Race {
    @JsonProperty(required = true)
    val name: String = ""

    @JsonProperty(required = true)
    val traits: Array<Result> = arrayOf()
    
    @JsonProperty(required = true)
    val description: String = ""
    
    override fun toString(): String {
        return "Race(name='$name', traits=${traits.contentToString()}, description='$description')"
    }
}
