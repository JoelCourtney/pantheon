package quantities

import java.lang.Exception

interface Quantity<U: Unit> {
    override fun toString(): String
}

enum class TimeKeyword(private val symbol: String) : Quantity<Time> {
    INSTANTANEOUS("instantaneous"),
    INDEFINITE("indefinite");

    override fun toString(): String {
        return symbol
    }
}

enum class DistanceKeyword(private val symbol: String) : Quantity<Distance> {
    TOUCH("touch");

    override fun toString(): String {
        return symbol
    }
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
        @JvmStatic fun fromString(s: String): QuantityBinaryOp {
            return when(s) {
                "+" -> ADD
                "-" -> SUBTRACT
                else -> throw Exception("Invalid operator.")
            }
        }
    }
}