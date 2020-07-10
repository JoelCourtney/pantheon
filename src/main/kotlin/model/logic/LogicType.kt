package model.logic

import com.fasterxml.jackson.annotation.JsonProperty
import model.identity.Evaluated

enum class LogicType(val identity: String) {
    @JsonProperty("any")
    ANY("any"),

    @JsonProperty("all")
    ALL("all"),

    @JsonProperty("consecutive")
    CONSECUTIVE("consecutive");
}
