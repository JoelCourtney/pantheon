package model.effects.results

import model.quantities.Damage

data class DamageResult(
    val damage: Damage
) : Result {
}