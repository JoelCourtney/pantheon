package model.triggers

import model.quantities.Damage
import model.quantities.DamageUnit

sealed class Trigger {
    object Now : Trigger()
    object StartOfTurnTrigger : Trigger()
    object EndOfTurnTrigger : Trigger()
    object StartOfActionTrigger : Trigger()
    object StartOfBonusActionTrigger : Trigger()
    object StartOfReactionTrigger : Trigger()
    object EndOfActionTrigger : Trigger()
    object EndOfBonusActionTrigger : Trigger()
    object EndOfReactionTrigger : Trigger()
    class TakeDamageTrigger(val `take damage`: Damage) : Trigger()
    class TakeDamageTypeTrigger(val `take damage type`: Array<DamageUnit>) : Trigger()
    class DealDamageTypeTrigger(val `deal damage type`: Array<DamageUnit>) : Trigger()
}
