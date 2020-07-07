package model.results.effects

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.quantities.damage.DamageUnit

data class GainResistanceEffect(
    @JsonProperty("gain resistance")
    val gainResistance: DamageUnit
) : Effect {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}