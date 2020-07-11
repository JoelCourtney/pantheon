package model.quantities

import model.quantities.QuantityType.Distance

enum class DistanceUnit(val identity: String): QuantityUnit<Distance> {
    FOOT("ft"),
    MILE("mi"),
    SPACE("sp");

    override fun toString(): String {
        return identity
    }
}
