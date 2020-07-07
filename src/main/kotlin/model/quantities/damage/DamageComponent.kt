package model.quantities.damage

import model.quantities.Expression

data class DamageComponent(val exp: Expression, val unit: DamageUnit):
    Damage {
    override fun toString(): String {
        return "$exp $unit"
    }
}