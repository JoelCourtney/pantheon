package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.logic.Trigger
import java.lang.IllegalArgumentException

data class TimedResult(
    val results: ArrayList<Result>,
    val until: Trigger = Trigger(),
    @JsonProperty("not until")
    val notUntil: Trigger = Trigger()
) : Result {
    init {
        if (until.isEmpty() and notUntil.isEmpty()) {
            throw IllegalArgumentException("Timed Results need at least one trigger")
        }
    }

    override fun purge(): Boolean =
        if (until.triggered) {
            true
        } else if (!notUntil.triggered) {
            false
        } else {
            results.removeAll { it.purge() }
            results.size == 0
        }

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
}