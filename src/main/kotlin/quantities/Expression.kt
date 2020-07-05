package quantities

import java.lang.Exception

open class Expression() {
}

class ExpressionBinary(
    val bop: ExpressionBinaryOp, val left: Expression, val right: Expression
) : Expression() {
}

enum class ExpressionBinaryOp {
    TIMES,
    DIVIDE_DOWN,
    DIVIDE_UP,
    ADD,
    SUBTRACT;

    companion object {
        @JvmStatic fun from_string(s: String): ExpressionBinaryOp {
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
    constructor(s: String) : this(s.toInt())
}

data class Dice(val n: Int, val size: Int) : Expression() {
    constructor(s1: String, s2: String) : this(s1.toInt(), s2.toInt())
}

data class Identifier(val s: String) : Expression() {

}