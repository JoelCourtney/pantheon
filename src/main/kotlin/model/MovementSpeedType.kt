package model

import com.fasterxml.jackson.annotation.JsonProperty

enum class MovementSpeedType(val symbol: String) {
    @JsonProperty("walking")
    WALKING("walking"),

    @JsonProperty("flying")
    FLYING("flying"),

    @JsonProperty("swimming")
    SWIMMING("swimming"),

    @JsonProperty("burrowing")
    BURROWING("burrowing"),

    @JsonProperty("climbing")
    CLIMBING("climbing");
}