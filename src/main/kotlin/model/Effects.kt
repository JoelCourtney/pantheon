package model

import model.utility.Effect

interface EffectGenerator {
    fun generateEffects(): Array<Effect>
}