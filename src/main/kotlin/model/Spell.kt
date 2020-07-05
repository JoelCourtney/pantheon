package model

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import java.lang.IllegalArgumentException

class Spell(
    val name: String,
    val level: Int
) {
    val school: SpellSchool = SpellSchool.NONE
}

enum class SpellSchool(val symbol: String) {
    EVOCATION("evocation"),
    NONE("none");
}

class SchoolDeserializer : StdDeserializer<SpellSchool>(SpellSchool::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): SpellSchool {
        val s = p?.readValueAs(String::class.java) ?: throw IllegalArgumentException("Parser is null")
        return SpellSchool.valueOf(s.toUpperCase())
    }
}