package quantities

class Quantity<U: Unit> {
    data class QuantityComponent<U: Unit>(val exp: Expression, val unit: U)

    val comps: Array<QuantityComponent<U>> = arrayOf()

    init {

    }
}