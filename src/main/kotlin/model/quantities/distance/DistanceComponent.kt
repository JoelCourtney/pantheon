package model.quantities.distance

import model.quantities.Expression

data class DistanceComponent(val exp: Expression, val unit: DistanceUnit):
    Distance {
    override fun toString(): String {
        return "$exp $unit"
    }
}