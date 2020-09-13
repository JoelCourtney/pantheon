package model.access

import model.quantities.amounts.AmountBinaryOp

interface Accessible {
    operator fun get(id: String): Any
    operator fun set(id: String, value: String)
//    fun modify(op: AmountBinaryOp, value: String)
}
