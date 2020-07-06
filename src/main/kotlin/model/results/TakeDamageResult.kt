package model.results

import model.Character
import model.quantities.Damage

data class TakeDamageResult(
    val `take damage`: Damage
) : Result {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}