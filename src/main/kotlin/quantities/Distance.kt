package quantities

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

interface Distance : Quantity

data class DistanceComponent(val exp: Expression, val unit: DistanceUnit): Distance {
    override fun toString(): String {
        return exp.toString() + " " + unit.symbol
    }
}

enum class DistanceUnit(override val symbol: String) : Unit {
    FOOT("ft"),
    MILE("mi"),
    SPACE("spc");
}

enum class DistanceKeyword(private val symbol: String) : Distance {
    TOUCH("touch"),
    SELF("self");

    override fun toString(): String {
        return symbol
    }
}

class DistanceDeserializer : StdDeserializer<Distance>(Distance::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Distance {
        val s = p!!.readValueAs(String::class.java)
        return ParseWrapper.parseDistance(s)
    }
}