package model.effects.results

import model.Character

interface Result {
    fun apply(c: Character)
}
