package model.results.effects

import model.Character
import model.results.Result

interface Effect : Result {
    fun apply(c: Character)
}