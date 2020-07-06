package model.results

import model.Character
import model.timers.Timer
import model.timers.TimerListType

class TimedResult(
    var timers: Array<Timer>,
    var `timer type`: TimerListType,
    var results: Array<Result>
) : Result {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
}