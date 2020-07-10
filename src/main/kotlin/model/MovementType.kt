package model

import com.fasterxml.jackson.annotation.JsonProperty
import model.identity.Evaluated

enum class MovementType(val identity: String): Evaluated<MovementType> {
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
