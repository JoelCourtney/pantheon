package model.quantities

import model.access.Expression

data class Quantity<T: QuantityType>(
    val terms: ArrayList<Expression<QuantityComponent<T>>>
): Expression<Quantity<T>> {
    override fun evaluate(): Quantity<T> {
        TODO("Not yet implemented")
    }
    
    companion object {
        inline fun<reified T: QuantityType> empty(): Quantity<T> =
            Quantity<T>(arrayListOf())
    }
}

//typealias Damage = Quantity<QuantityType.Damage>
//typealias Distance = Quantity<QuantityType.Distance>
//typealias Time = Quantity<QuantityType.Time>
