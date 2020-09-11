package model.access

import model.quantities.amounts.AmountBinaryOp

interface Accessible {
    operator fun get(id: String): Accessible
//    fun modify(op: AmountBinaryOp, value: String)
}
