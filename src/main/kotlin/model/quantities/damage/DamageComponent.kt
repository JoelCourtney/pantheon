package model.quantities.damage

import model.quantities.amounts.Amount

data class DamageComponent(val exp: Amount, val unit: DamageUnit) : Damage {
    override fun toString(): String {
        return "$exp $unit"
    }
}