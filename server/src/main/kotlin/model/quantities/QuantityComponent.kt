package model.quantities

import model.access.Expression
import model.quantities.amounts.Amount

data class QuantityComponent<T: QuantityType>(val amount: Expression<Amount>, val unit: Expression<QuantityUnit<T>>):
    Expression<QuantityComponent<T>> {
    override fun evaluate(): QuantityComponent<T> {
        TODO("Not yet implemented")
    }
    
    override fun toString(): String {
        return "$amount $unit"
    }
}
