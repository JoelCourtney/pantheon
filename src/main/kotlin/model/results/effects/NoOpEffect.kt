package model.results.effects

import model.gameObjects.Character

object NoOpEffect : Effect() {
    override fun applyEffect(c: Character) {}
}
