package model.modifications.results

import Engine
import model.gameObjects.Character
import model.logic.Event
import model.modifications.effects.Effect

data class SpawnEventResult(
    val event: Event
): Result {
    override fun applyResult(e: Engine): Effect? {
        TODO("Not yet implemented")
    }

    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }
}
