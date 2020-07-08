package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.MovementSpeedType
import model.quantities.damage.DamageUnit
import model.quantities.distance.Distance
import model.results.effects.Effect

data class BaseSpeedStatus(
    @JsonProperty("base movement speed")
    val speedType: MovementSpeedType,
    val distance: Distance
) : Status() {
    override fun applyStatus(c: Character) {
        TODO("Not yet implemented")
    }
}