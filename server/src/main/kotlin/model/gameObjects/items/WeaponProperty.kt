package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty

enum class WeaponProperty {
    @JsonProperty("ammunition")
    AMMUNITION,

    @JsonProperty("finesse")
    FINESSE,

    @JsonProperty("heavy")
    HEAVY,

    @JsonProperty("light")
    LIGHT,

    @JsonProperty("loading")
    LOADING,

    @JsonProperty("reach")
    REACH,

    @JsonProperty("thrown")
    THROWN,

    @JsonProperty("two handed")
    TWO_HANDED,

    @JsonProperty("versatile")
    VERSATILE,

    @JsonProperty("improvised")
    IMPROVISED,

    @JsonProperty("silvered")
    SILVERED,

    @JsonProperty("special")
    SPECIAL;
}