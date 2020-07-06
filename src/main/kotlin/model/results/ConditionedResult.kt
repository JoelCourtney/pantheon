package model.results

import model.Character
import model.conditions.Condition
import model.conditions.ConditionListType

class ConditionedResult(
    var conditions: Array<Condition>,
    var `condition type`: ConditionListType,
    var results: Array<Result>
) : Result {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}