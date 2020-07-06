package model.results

import model.Character
import model.quantities.DamageUnit

class GainResistance(
    val resistance: DamageUnit
) : Result {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}