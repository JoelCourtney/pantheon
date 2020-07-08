package model.results

import com.fasterxml.jackson.annotation.JsonProperty

enum class SavingThrowType(val symbol: String) {
    @JsonProperty("strength")
    STRENGTH("strength"),

    @JsonProperty("dexterity")
    DEXTERITY("dexterity"),

    @JsonProperty("constitution")
    CONSTITUTION("constitution"),

    @JsonProperty("intelligence")
    INTELLIGENCE("intelligence"),

    @JsonProperty("wisdom")
    WISDOM("wisdom"),

    @JsonProperty("charisma")
    CHARISMA("charisma"),

    @JsonProperty("death")
    DEATH("death");
}
