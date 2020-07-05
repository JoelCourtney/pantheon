package quantities

import java.lang.Exception

open class Quantity<U: Unit>

data class QuantityComponent<U: Unit>(val exp: Expression, val unit: U): Quantity<U>()

class QuantityBinary<U: Unit>(
    val bop: QuantityBinaryOp, val left: Quantity<U>, val right: Quantity<U>
): Quantity<U>() {

}

enum class QuantityBinaryOp {
    ADD,
    SUBTRACT;

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