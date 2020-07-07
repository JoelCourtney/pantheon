package model.results.effects

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.quantities.damage.Damage

data class DealExtraDamageEffect(
    @JsonProperty("deal extra damage")
    val dealExtraDamage: Damage
) : Effect {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}