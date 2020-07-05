package model.effects.results

import model.Character

object NoOpResult : Result {
    override fun apply(c: Character) {}
}
