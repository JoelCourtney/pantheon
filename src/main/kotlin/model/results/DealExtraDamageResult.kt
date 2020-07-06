package model.results

import model.Character
import model.quantities.Damage

class DealExtraDamageResult(
    val damage: Damage
) : Result {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}