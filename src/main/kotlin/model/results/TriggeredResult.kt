package model.results

import model.Character
import model.triggers.Trigger
import model.triggers.TriggerListType

class TriggeredResult(
    var triggers: Array<Trigger>,
    var `trigger type`: TriggerListType,
    var results: Array<Result>
) : Result {
    var `limited triggers`: Int = 1

    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}