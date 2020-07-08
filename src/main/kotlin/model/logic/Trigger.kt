package model.logic

import model.quantities.time.Time

data class Trigger(
    val events: Array<Event> = arrayOf(),
    val timers: Array<Timer> = arrayOf(),
    val type: LogicType = LogicType.CONSECUTIVE
) {
    var triggered: Boolean = false
        private set

    fun processEvent(e: Event): Boolean {
        TODO("asdf")
    }

    fun processTime(t: Time): Boolean {
        TODO("zxcv")
    }

    fun isEmpty(): Boolean {
        return events.isEmpty() && timers.isEmpty()
    }
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Trigger

        if (!events.contentEquals(other.events)) return false
        if (!timers.contentEquals(other.timers)) return false
        if (type != other.type) return false

        return true
    }

    override fun hashCode(): Int {
        var result = events.contentHashCode()
        result = 31 * result + timers.contentHashCode()
        result = 31 * result + type.hashCode()
        return result
    }
}