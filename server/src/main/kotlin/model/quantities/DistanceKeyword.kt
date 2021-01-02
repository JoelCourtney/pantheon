package model.quantities

import model.access.Expression
import model.quantities.QuantityType.Distance

enum class DistanceKeyword(val identity: String): Expression<QuantityComponent<Distance>> {
    TOUCH("touch"),
    SELF("self");

    override fun evaluate(): QuantityComponent<Distance> {
        TODO("Not yet implemented")
    }

    override fun toString(): String {
        return identity
    }
}
