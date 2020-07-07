package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.logic.Trigger

data class TimedResult(
    val results: Array<Result>,
    val until: Trigger = Trigger(),
    @JsonProperty("not until")
    val notUntil: Trigger = Trigger()
) : Result {
    override val isResolved: Boolean = false

    override fun apply(c: Character) {
        TODO("Not yet implemented")
        /*

        implementation notes for later (very important):

        if not T R I G G E R E D:
            sit there quietly like a good little object
        else:
            DRAMATICALLY SPILL FEELINGS OUT OVER DINNER like an emotionally repressed little object

         */
    }

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