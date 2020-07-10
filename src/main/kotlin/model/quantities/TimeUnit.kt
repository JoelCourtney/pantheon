package model.quantities

import model.identity.Evaluated

enum class TimeUnit(val identity: String): QuantityUnit<QuantityType.Time> {
    ACTION("action"),
    BONUS_ACTION("bonus action"),
    REACTION("reaction"),
    ROUND("round"),
    SECOND("second"),
    MINUTE("minute"),
    HOUR("hour"),
    DAY("day"),
    LONG_REST("long rest"),
    SHORT_REST("short rest");
    
    override fun toString(): String {
        return identity
    }
}
