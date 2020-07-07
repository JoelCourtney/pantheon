package model.results

import model.SavingThrowType
import model.quantities.amounts.Amount

class SavingThrowResult(
    val `saving throw`: SavingThrowType,
    val dc: Amount
) : Result {
    val `fail results`: Array<Result> = arrayOf()
    val `success results`: Array<Result> = arrayOf()
    val `half damage on success`: Boolean = false
}