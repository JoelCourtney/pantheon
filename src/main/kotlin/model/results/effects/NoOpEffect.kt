package model.results.effects

import model.Character
import model.results.Result

object NoOpEffect : Effect {
    override fun apply(c: Character) {}
}
