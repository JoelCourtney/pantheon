package quantities

import java.lang.Exception

interface Quantity<U: Unit> {
    override fun toString(): String
}

data class QuantityComponent<U: Unit>(val exp: Expression, val unit: U): Quantity<U> {
    override fun toString(): String {
        return exp.toString() + " " + unit.symbol
    }
}

class QuantityBinary<U: Unit>(
    val bop: QuantityBinaryOp, val left: Quantity<U>, val right: Quantity<U>
): Quantity<U> {
    override fun toString(): String {
        return "$left ${bop.symbol} $right"
    }
}

enum class QuantityBinaryOp(val symbol: String) {
    ADD("+"),
    SUBTRACT("-");

    companion object {
        @JvmStatic fun from_string(s: String): QuantityBinaryOp {
            return when(s) {
                "+" -> ADD
                "-" -> SUBTRACT
                else -> throw Exception("Invalid operator.")
            }
        }
    }
}