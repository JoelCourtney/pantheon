package model.effects.results

import model.Character
import model.quantities.Damage

data class DamageResult(
    val damage: Damage
) : Result {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}