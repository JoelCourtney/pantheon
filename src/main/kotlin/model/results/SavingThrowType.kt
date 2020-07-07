package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

enum class SavingThrowType {
    @JsonProperty("strength")
    STRENGTH,

    @JsonProperty("dexterity")
    DEXTERITY,

    @JsonProperty("constitution")
    CONSTITUTION,

    @JsonProperty("intelligence")
    INTELLIGENCE,

    @JsonProperty("wisdom")
    WISDOM,

    @JsonProperty("charisma")
    CHARISMA,

    @JsonProperty("death")
    DEATH;
}

