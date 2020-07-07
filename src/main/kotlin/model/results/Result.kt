package model.results

import model.Character

interface Result {
    val isResolved: Boolean
    fun apply(c: Character)
}
