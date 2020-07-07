package model.lifetimes

import model.logic.Timer
import model.logic.Event
import model.logic.LogicType

class Trigger {
    val events: Array<Event> = arrayOf()
    val timers: Array<Timer> = arrayOf()
    val type: LogicType = LogicType.CONSECUTIVE
}