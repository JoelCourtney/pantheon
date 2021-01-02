package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty

enum class ArmorType {
    @JsonProperty("light")
    LIGHT,

    @JsonProperty("medium")
    MEDIUM,

    @JsonProperty("heavy")
    HEAVY
}