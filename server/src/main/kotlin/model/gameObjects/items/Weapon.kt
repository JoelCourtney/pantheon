package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DamageDeserializer
import io.deserializers.DistanceDeserializer
import model.quantities.Quantity
import model.quantities.QuantityType

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
    val range: Quantity<QuantityType.Distance>,
    @JsonDeserialize(using = DistanceDeserializer::class)
    @JsonProperty("disadvantage range")
    val disadvantageRange: Quantity<QuantityType.Distance>,
    @JsonDeserialize(using = DamageDeserializer::class)
    val damage: Quantity<QuantityType.Damage>
): Item(name, rarity, weight, price, magical, description, false) {
}
