package model.results

import model.Character
import model.SavingThrowType
import model.quantities.Expression

class SavingThrowResult(
    val `saving throw`: SavingThrowType,
    val dc: Expression
) : Result {
    val `fail results`: Array<Result> = arrayOf()
    val `success results`: Array<Result> = arrayOf()
    val `half damage on success`: Boolean = false
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}