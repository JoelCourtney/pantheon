package model.results.effects

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.quantities.damage.Damage

data class TakeDamageEffect(
    @JsonProperty("take damage")
    val takeDamage: Damage
) : Effect {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}