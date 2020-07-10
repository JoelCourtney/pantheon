package model.spells

import com.fasterxml.jackson.annotation.JsonProperty
import model.identity.Evaluated

enum class SpellSchool(val identity: String): Evaluated<SpellSchool> {
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
