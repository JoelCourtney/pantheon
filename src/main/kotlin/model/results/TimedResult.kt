package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.logic.Trigger

data class TimedResult(
    val results: Array<Result>,
    val until: Trigger = Trigger(),
    @JsonProperty("not until")
    val notUntil: Trigger = Trigger()
) : Result {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as TimedResult

        if (!results.contentEquals(other.results)) return false
        if (until != other.until) return false
        if (notUntil != other.notUntil) return false

        return true
    }

    override fun hashCode(): Int {
        var result = results.contentHashCode()
        result = 31 * result + until.hashCode()
        result = 31 * result + notUntil.hashCode()
        return result
    }
}