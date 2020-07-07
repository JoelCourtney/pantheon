package model.quantities.time

import model.quantities.Expression

data class TimeComponent(val exp: Expression, val unit: TimeUnit):
    Time {
    override fun toString(): String {
        return "$exp $unit"
    }
}