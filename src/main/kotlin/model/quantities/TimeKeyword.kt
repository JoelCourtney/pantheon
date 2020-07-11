package model.quantities

import model.access.Expression
import model.quantities.QuantityType.Time

enum class TimeKeyword(val identity: String): Expression<QuantityComponent<Time>> {
    INSTANTANEOUS("instantaneous"),
    INDEFINITE("indefinite"),
    NOW("now");

    override fun evaluate(): QuantityComponent<Time> {
        TODO("Not yet implemented")
    }

    override fun toString(): String {
        return identity
    }
}
