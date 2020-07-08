package model.races

import com.fasterxml.jackson.annotation.JsonProperty
import model.results.Result

open class Race {
    @JsonProperty(required = true)
    val name: String = ""

    @JsonProperty(required = true)
    val results: Array<Result> = arrayOf()

    override fun toString(): String {
        return "Race(name='$name', results=${results.contentToString()})"
    }
}