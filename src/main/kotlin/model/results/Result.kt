package model.results

import model.Character

interface Result {
    fun apply(c: Character)
    fun purge(): Boolean
}
