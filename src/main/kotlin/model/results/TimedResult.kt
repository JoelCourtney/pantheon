package model.results

import model.lifetimes.Trigger

class TimedResult(
    val results: Array<Result>
) : Result {
    val until: Trigger = Trigger()
    val `not until`: Trigger = Trigger()
}