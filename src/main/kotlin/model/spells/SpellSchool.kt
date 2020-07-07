package model.spells

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

enum class SpellSchool(val symbol: String) {
    @JsonProperty("abjuration")
    ABJURATION("abjuration"),

    @JsonProperty("conjuration")
    CONJURATION("conjuration"),

    @JsonProperty("divination")
    DIVINATION("divination"),

    @JsonProperty("enchantment")
    ENCHANTMENT("enchantment"),

    @JsonProperty("evocation")
    EVOCATION("evocation"),

    @JsonProperty("illusion")
    ILLUSION("illusion"),

    @JsonProperty("necromancy")
    NECROMANCY("necromancy"),

    @JsonProperty("transmutation")
    TRANSMUTATION("transmutation"),

    @JsonProperty("universal")
    UNIVERSAL("universal");
}

//class SpellSchoolDeserializer : StdDeserializer<SpellSchool>(SpellSchool::class.java) {
//    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): SpellSchool {
//        val s = p!!.readValueAs(String::class.java)
//        return SpellSchool.valueOf(s.toUpperCase())
//    }
//}