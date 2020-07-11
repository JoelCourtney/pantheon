package model.results.effects

import model.gameObjects.Character
import model.logic.Event

data class SpawnEventEffect(
    val event: Event
): Effect() {
    override fun applyEffect(c: Character) {
        TODO("Not yet implemented")
    }
}
