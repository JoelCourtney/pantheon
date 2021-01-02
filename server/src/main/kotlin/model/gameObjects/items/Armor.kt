package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeName

@JsonTypeName("armor")
class Armor(
    name: String,
    rarity: Rarity,
    weight: Double,
    price: Double,
    magical: Boolean,
    description: String,
    @JsonProperty("subtype")
    val type: ArmorType,
    val ac: Int
): Item(name, rarity, weight, price, magical, description, false) {
}