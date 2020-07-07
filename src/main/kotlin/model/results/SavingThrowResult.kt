package model.results

import model.quantities.amounts.Amount

data class SavingThrowResult(
    val `saving throw`: SavingThrowType,
    val dc: Amount,
    val `fail results`: Array<Result> = arrayOf(),
    val `success results`: Array<Result> = arrayOf(),
    val `half damage on success`: Boolean = false
) : Result {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as SavingThrowResult

        if (`saving throw` != other.`saving throw`) return false
        if (dc != other.dc) return false
        if (!`fail results`.contentEquals(other.`fail results`)) return false
        if (!`success results`.contentEquals(other.`success results`)) return false
        if (`half damage on success` != other.`half damage on success`) return false

        return true
    }

    override fun hashCode(): Int {
        var result = `saving throw`.hashCode()
        result = 31 * result + dc.hashCode()
        result = 31 * result + `fail results`.contentHashCode()
        result = 31 * result + `success results`.contentHashCode()
        result = 31 * result + `half damage on success`.hashCode()
        return result
    }
}
