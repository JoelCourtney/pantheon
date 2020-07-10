package model.quantities

sealed class QuantityType {
    object Damage: QuantityType()
    object Distance: QuantityType()
    object Time: QuantityType()
}
