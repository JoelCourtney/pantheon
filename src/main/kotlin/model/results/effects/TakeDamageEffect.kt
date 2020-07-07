package model.results.effects

import model.Character
import model.quantities.damage.Damage

data class TakeDamageEffect(
    val `take damage`: Damage
) : Effect {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}