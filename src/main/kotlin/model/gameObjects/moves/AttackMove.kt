package model.gameObjects.moves

import model.quantities.Quantity
import model.quantities.QuantityType

data class AttackMove(
    val weaponName: String,
    val damage: Quantity<QuantityType.Damage>,
    val reach: Quantity<QuantityType.Distance>,
    val maxRange: Quantity<QuantityType.Distance>?,
    val dc: Int,
    val ranged: Boolean
): Move