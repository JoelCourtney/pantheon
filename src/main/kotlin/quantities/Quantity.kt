package quantities

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import java.lang.Exception

interface Quantity

class QuantityBinary(
    val bop: QuantityBinaryOp, val left: Quantity, val right: Quantity
): Distance, Damage, Time {
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
