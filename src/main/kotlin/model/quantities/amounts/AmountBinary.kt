package model.quantities.amounts

import model.identity.Expression

data class AmountBinary(
    val bop: AmountBinaryOp, val left: Expression<Amount>, val right: Expression<Amount>
): Amount {
    override fun toString(): String {
        return left.toString() + bop.symbol + right.toString()
    }
}
