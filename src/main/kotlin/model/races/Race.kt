package model.races

import com.fasterxml.jackson.annotation.JsonProperty

open class Race {
    @JsonProperty(required = true)
    var name: String = ""
}