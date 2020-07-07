package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.quantities.damage.DamageUnit
import model.results.effects.Effect

data class HaveResistanceStatus(
    @JsonProperty("have resistance")
    val gainResistance: DamageUnit
) : Status {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}