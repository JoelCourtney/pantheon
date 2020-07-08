package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import model.AbilityScoreType
import model.Character
import model.quantities.amounts.Amount

data class AbilityScoreIncreaseStatus(
    @JsonProperty("ability score increase")
    val abilityScore: AbilityScoreType,
    val amount: Amount
) : Status() {
    override fun applyStatus(c: Character) {
        TODO("Not yet implemented")
    }
}