package model.quantities

import model.quantities.damage.Damage
import model.quantities.distance.Distance
import model.quantities.time.Time
import java.lang.Exception

interface Quantity

data class QuantityBinary(
    val left: Quantity, val right: Quantity
): Distance, Damage, Time {
    override fun toString(): String {
        return "$left; $right"
    }
}
