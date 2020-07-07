package model.quantities.time

enum class TimeUnitLiteral(val symbol: String) : TimeUnit {
    ACTION("action"),
    BONUS_ACTION("bonus action"),
    REACTION("reaction"),
    ROUND("round"),
    SECOND("sec"),
    MINUTE("min"),
    HOUR("hr"),
    DAY("day"),
    LONG_REST("long rest"),
    SHORT_REST("short rest");

    override fun getTimeUnitLiteral(): TimeUnitLiteral {
        return this
    }
}