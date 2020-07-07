package model.quantities.time

import model.quantities.amounts.Amount

data class TimeComponent(val exp: Amount, val unit: TimeUnit) : Time {
    override fun toString(): String {
        return "$exp $unit"
    }
}