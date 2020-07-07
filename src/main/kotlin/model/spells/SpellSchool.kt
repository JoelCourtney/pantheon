package model.spells

import com.fasterxml.jackson.annotation.JsonProperty

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
