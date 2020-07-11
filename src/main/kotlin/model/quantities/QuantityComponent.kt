package model.quantities

import model.quantities.amounts.Amount
import model.access.Expression

data class QuantityComponent<T: QuantityType>(val amount: Expression<Amount>, val unit: Expression<QuantityUnit<T>>):
    Expression<QuantityComponent<T>> {
    override fun evaluate(): QuantityComponent<T> {
        TODO("Not yet implemented")
    }
    
    override fun toString(): String {
        return "$amount $unit"
    }
}
