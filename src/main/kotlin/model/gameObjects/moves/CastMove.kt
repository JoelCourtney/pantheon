package model.gameObjects.moves

import model.quantities.Quantity
import model.quantities.QuantityType

data class CastMove(
    val spellName: String,
    val range: Quantity<QuantityType.Distance>,
    val dc: Int,
    val damage: Quantity<QuantityType.Damage>
): Move