package model.quantities.amounts

import java.lang.Exception

enum class AmountBinaryOp(val symbol: String) {
    TIMES("*"),
    DIVIDE_DOWN("/"),
    DIVIDE_UP("/^"),
    ADD("+"),
    SUBTRACT("-");



    companion object {
        @JvmStatic fun fromString(s: String): AmountBinaryOp {
            return when (s) {
                "*" -> TIMES
                "/" -> DIVIDE_DOWN
                "/^" -> DIVIDE_UP
                "+" -> ADD
                "-" -> SUBTRACT
                else -> throw Exception("Invalid operator")
            }
        }
    }
}