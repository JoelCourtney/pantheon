package model.quantities.distance

import model.quantities.amounts.Amount

data class DistanceComponent(val exp: Amount, val unit: DistanceUnit) : Distance {
    override fun toString(): String {
        return "$exp $unit"
    }
}