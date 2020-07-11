package model.results

import model.gameObjects.Character

interface Result {
    fun apply(c: Character)
    fun purge(): Boolean
}
