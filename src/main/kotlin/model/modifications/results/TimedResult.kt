package model.modifications.results

import Engine
import com.fasterxml.jackson.annotation.JsonProperty
import model.gameObjects.Character
import model.logic.Trigger
import model.modifications.Modification
import model.modifications.effects.Effect
import java.lang.IllegalArgumentException

data class TimedResult(
        val results: ArrayList<Result>,
        val until: Trigger = Trigger(),
        @JsonProperty("not until")
    val notUntil: Trigger = Trigger()
): Result {
    init {
        if (until.isEmpty() and notUntil.isEmpty()) {
            throw IllegalArgumentException("Timed Results need at least one trigger")
        }
    }

    override fun applyResult(e: Engine): Effect? {
        TODO("Not yet implemented")
    }

    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }
}
