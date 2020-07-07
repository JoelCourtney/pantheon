package model.results.effects

import model.Character

object NoOpEffect : Effect {
    override fun apply(c: Character) {}
}
