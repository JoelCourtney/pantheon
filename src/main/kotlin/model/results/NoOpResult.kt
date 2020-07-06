package model.results

import model.Character

object NoOpResult : Result {
    override fun apply(c: Character) {}
}
