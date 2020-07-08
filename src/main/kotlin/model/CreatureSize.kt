package model

import com.fasterxml.jackson.annotation.JsonProperty

enum class CreatureSize {
    @JsonProperty("tiny")
    TINY,

    @JsonProperty("small")
    SMALL,

    @JsonProperty("medium")
    MEDIUM,

    @JsonProperty("large")
    LARGE,

    @JsonProperty("huge")
    HUGE,

    @JsonProperty("gargantuan")
    GARGANTUAN;
}