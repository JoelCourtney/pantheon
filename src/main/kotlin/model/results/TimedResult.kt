package model.results

import model.lifetimes.Trigger

data class TimedResult(
    val results: Array<Result>,
    val until: Trigger = Trigger(),
    val `not until`: Trigger = Trigger()
) : Result {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as TimedResult

        if (!results.contentEquals(other.results)) return false
        if (until != other.until) return false
        if (`not until` != other.`not until`) return false

        return true
    }

    override fun hashCode(): Int {
        var result = results.contentHashCode()
        result = 31 * result + until.hashCode()
        result = 31 * result + `not until`.hashCode()
        return result
    }
}