package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DamageDeserializer
import io.deserializers.DistanceDeserializer
import model.quantities.Damage
import model.quantities.Distance

@JsonTypeName("weapon")
class Weapon(
    name: String,
    rarity: Rarity,
    weight: Double,
    price: Double,
    magical: Boolean,
    description: String,
    @JsonProperty("subtype")
    val type: WeaponType,
    val properties: List<WeaponProperty>,
    @JsonDeserialize(using = DistanceDeserializer::class)
    val range: Distance,
    @JsonDeserialize(using = DistanceDeserializer::class)
    @JsonProperty("disadvantage range")
    val disadvantageRange: Distance,
    @JsonDeserialize(using = DamageDeserializer::class)
    val damage: Damage
): Item(name, rarity, weight, price, magical, description, false) {
}
