package model.gameObjects.items

import model.quantities.Quantity
import model.quantities.QuantityType

class Weapon(
    name: String,
    weight: Double,
    cost: Double,
    rarity: Rarity,
    intrinsic: Boolean,
    magical: Boolean,
    type: WeaponType,
    properties: List<WeaponProperty>,
    range: Quantity<QuantityType.Distance>,
    disadvantageRange: Quantity<QuantityType.Distance>,
    damage: Quantity<QuantityType.Damage>
): Item(name, weight, cost, rarity, intrinsic, magical) {
}
