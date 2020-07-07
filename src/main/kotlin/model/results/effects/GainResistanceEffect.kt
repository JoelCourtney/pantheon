package model.results.effects

import model.Character
import model.quantities.damage.DamageUnit

data class GainResistanceEffect(
    val `gain resistance`: DamageUnit
) : Effect {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}