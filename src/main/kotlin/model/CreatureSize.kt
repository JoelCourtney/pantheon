package model

import com.fasterxml.jackson.annotation.JsonProperty
import model.access.Evaluated

enum class CreatureSize(val identity: String): Evaluated<CreatureSize> {
    @JsonProperty("tiny")
    TINY("tiny"),

    @JsonProperty("small")
    SMALL("small"),

    @JsonProperty("medium")
    MEDIUM("medium"),

    @JsonProperty("large")
    LARGE("large"),

    @JsonProperty("huge")
    HUGE("huge"),

    @JsonProperty("gargantuan")
    GARGANTUAN("gargantuan");
}
