package model.spells

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import java.lang.IllegalArgumentException

enum class SpellSchool(val symbol: String) {
    ABJURATION("abjuration"),
    CONJURATION("conjuration"),
    DIVINATION("divination"),
    ENCHANTMENT("enchantment"),
    EVOCATION("evocation"),
    ILLUSION("illusion"),
    NECROMANCY("necromancy"),
    TRANSMUTATION("transmutation"),
    UNIVERSAL("universal")
}

class SpellSchoolDeserializer : StdDeserializer<SpellSchool>(SpellSchool::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): SpellSchool {
        val s = p!!.readValueAs(String::class.java)
        return SpellSchool.valueOf(s.toUpperCase())
    }
}