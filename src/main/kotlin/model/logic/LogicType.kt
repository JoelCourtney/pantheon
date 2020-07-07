package model.logic

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

enum class LogicType {
    @JsonProperty("any")
    ANY,

    @JsonProperty("all")
    ALL,

    @JsonProperty("consecutive")
    CONSECUTIVE;
}
