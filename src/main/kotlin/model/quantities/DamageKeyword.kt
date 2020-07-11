package model.quantities

import model.access.Expression
import model.quantities.QuantityType.Damage

enum class DamageKeyword(val identity: String): Expression<QuantityComponent<Damage>> {
    NONE("none");

    override fun evaluate(): QuantityComponent<Damage> {
        TODO("Not yet implemented")
    }

    override fun toString(): String {
        return identity
    }
}
