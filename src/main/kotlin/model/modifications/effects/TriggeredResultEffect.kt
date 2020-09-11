package model.modifications.effects

import Engine
import model.logic.Trigger
import model.modifications.results.Result

class TriggeredResultEffect(
        val trigger: Trigger,
        val results: List<Result>
): Effect {
    override fun applyEffect(e: Engine) {
        TODO("Not yet implemented")
    }
}