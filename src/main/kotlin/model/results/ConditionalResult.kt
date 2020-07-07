package model.results

import model.logic.LogicType
import model.logic.Question

data class ConditionalResult(
    val `if`: Array<Question> = arrayOf(),
    val `if not`: Array<Question> = arrayOf(),

    val `success results`: Array<Result> = arrayOf(),
    val `failure results`: Array<Result> = arrayOf()

) : Result {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ConditionalResult

        if (!`if`.contentEquals(other.`if`)) return false
        if (!`if not`.contentEquals(other.`if not`)) return false
        if (!`success results`.contentEquals(other.`success results`)) return false
        if (!`failure results`.contentEquals(other.`failure results`)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = `if`.contentHashCode()
        result = 31 * result + `if not`.contentHashCode()
        result = 31 * result + `success results`.contentHashCode()
        result = 31 * result + `failure results`.contentHashCode()
        return result
    }
}