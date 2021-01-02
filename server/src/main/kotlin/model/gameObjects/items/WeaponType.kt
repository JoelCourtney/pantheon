package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize

enum class WeaponType {
    @JsonProperty("simple")
    SIMPLE,

    @JsonProperty("martial")
    MARTIAL,

    @JsonProperty("firearm")
    FIREARM;
}