package model.logic

import com.fasterxml.jackson.annotation.JsonProperty

enum class LogicType(val symbol: String) {
    @JsonProperty("any")
    ANY("any"),

    @JsonProperty("all")
    ALL("all"),

    @JsonProperty("consecutive")
    CONSECUTIVE("consecutive");
}
