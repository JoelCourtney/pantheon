package quantities

import java.lang.Exception

interface Expression {
    override fun toString(): String
}

class ExpressionBinary(
    val bop: ExpressionBinaryOp, val left: Expression, val right: Expression
) : Expression {
    override fun toString(): String {
        return left.toString() + bop.symbol + right.toString()
    }
}

enum class ExpressionBinaryOp(val symbol: String) {
    TIMES("*"),
    DIVIDE_DOWN("/-"),
    DIVIDE_UP("/+"),
    ADD("+"),
    SUBTRACT("-");



    companion object {
        @JvmStatic fun from_string(s: String): ExpressionBinaryOp {
            println(s)
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

data class NumberLiteral(val n: Int) : Expression {
    constructor(s: String) : this(s.toInt())

    override fun toString(): String {
        return n.toString()
    }
}

data class Dice(val n: Int, val size: Int) : Expression {
    constructor(s1: String, s2: String) : this(s1.toInt(), s2.toInt())

    override fun toString(): String {
        return n.toString() + "d" + size.toString()
    }
}

data class Identifier(val s: String) : Expression {
    override fun toString(): String {
        return s
    }
}