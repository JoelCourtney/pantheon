package model.triggers

import model.quantities.Damage
import model.quantities.DamageUnit

sealed class Trigger {
    class TakeDamageTrigger(d: Damage) : Trigger()
    class TakeDamageTypeTrigger(t: Array<DamageUnit>) : Trigger()
    object StartOfTurnTrigger : Trigger()
    object EndOfTurnTrigger : Trigger()
    object Now : Trigger()
    object StartOfActionTrigger : Trigger()
    object StartOfBonusActionTrigger : Trigger()
    object StartOfReactionTrigger : Trigger()
    object EndOfActionTrigger : Trigger()
    object EndOfBonusActionTrigger : Trigger()
    object EndOfReactionTrigger : Trigger()
}
