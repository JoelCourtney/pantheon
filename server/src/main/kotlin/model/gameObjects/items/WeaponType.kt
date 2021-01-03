package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty

enum class WeaponType {
    @JsonProperty("simple")
    SIMPLE,

    @JsonProperty("martial")
    MARTIAL,

    @JsonProperty("firearm")
    FIREARM;
}