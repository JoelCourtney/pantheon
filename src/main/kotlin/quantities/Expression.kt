package quantities

import java.lang.Exception

open class Expression() {
}

class BinaryOp(
    val bop: Binary, val left: Expression, val right: Expression
) : Expression() {
}

enum class Binary {
    TIMES,
    DIVIDE_DOWN,
    DIVIDE_UP,
    ADD,
    SUBTRACT;

    companion object {
        fun from_string(s: String): Binary {
            return when (s) {
                "*" -> TIMES
                "/", "/-" -> DIVIDE_DOWN
                "/+" -> DIVIDE_UP
                "+" -> ADD
                "-" -> SUBTRACT
                else -> throw Exception("Invalid operator")
            }
        }
    }
}

data class NumberLiteral(val n: Int) : Expression() {

}

data class Dice(val n: Int, val size: Int) : Expression() {

}

data class Identifier(val s: String) : Expression() {

}