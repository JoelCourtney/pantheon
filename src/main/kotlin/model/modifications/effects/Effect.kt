package model.modifications.effects

import Engine
import model.modifications.Modification
import model.gameObjects.Character

interface Effect: Modification {
    fun applyEffect(e: Engine)
}
