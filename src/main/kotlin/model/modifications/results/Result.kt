package model.modifications.results

import Engine
import model.gameObjects.Character
import model.modifications.Modification
import model.modifications.effects.Effect

interface Result: Modification {
    fun applyResult(e: Engine): Effect?
    fun purge(): Boolean
}
