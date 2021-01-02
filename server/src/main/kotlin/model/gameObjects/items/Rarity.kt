package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty

enum class Rarity {
    @JsonProperty("common")
    COMMON,

    @JsonProperty("uncommon")
    UNCOMMON,

    @JsonProperty("rare")
    RARE,

    @JsonProperty("very rare")
    VERY_RARE,

    @JsonProperty("legendary")
    LEGENDARY;

    companion object {
        fun fromString(s: String): Rarity {
            return Rarity.valueOf(s.toUpperCase().replace(' ','_'))
        }
    }
}