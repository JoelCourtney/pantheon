package model.quantities

import model.quantities.amounts.Amount
import model.quantities.damage.Damage
import model.quantities.damage.DamageUnit
import model.quantities.damage.DamageUnitLiteral
import model.quantities.distance.Distance
import model.quantities.distance.DistanceUnit
import model.quantities.distance.DistanceUnitLiteral
import model.quantities.time.Time
import model.quantities.time.TimeUnit
import model.quantities.time.TimeUnitLiteral

class Identifier(val s: String, val t: String? = null) :
    Amount,
    TimeUnit,
    DistanceUnit,
    DamageUnit,
    Time,
    Distance,
    Damage {

    override fun getDamageUnitLiteral(): DamageUnitLiteral {
        TODO("Not yet implemented")
    }

    override fun getDistanceUnitLiteral(): DistanceUnitLiteral {
        TODO("Not yet implemented")
    }

    override fun getTimeUnitLiteral(): TimeUnitLiteral {
        TODO("Not yet implemented")
    }

    override fun toString(): String {
        return if (t != null) {
            "$s$$t"
        } else {
            s
        }
    }
}