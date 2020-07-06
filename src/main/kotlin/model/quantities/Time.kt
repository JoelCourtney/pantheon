package model.quantities

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

interface Time : Quantity

data class TimeComponent(val exp: Expression, val unit: TimeUnit): Time {
    override fun toString(): String {
        return exp.toString() + " " + unit.symbol
    }
}

enum class TimeUnit(override val symbol: String) : Unit {
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
}

enum class TimeKeyword(private val symbol: String) : Time {
    INSTANTANEOUS("instantaneous"),
    INDEFINITE("indefinite");

    override fun toString(): String {
        return symbol
    }
}

class TimeDeserializer : StdDeserializer<Time>(Time::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Time {
        val s = p!!.readValueAs(String::class.java)
        return ParseWrapper.parseTime(s)
    }
}