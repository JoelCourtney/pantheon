package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.quantities.damage.Damage

data class MeleeWeaponStatus(
    @JsonProperty("have melee weapon")
    val name: String,
    val damage: Damage
): Status() {
    override fun applyStatus(c: Character) {
        TODO("Not yet implemented")
    }
}
