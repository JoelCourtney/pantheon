package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.identity.Evaluated

enum class SavingThrowType(val identity: String):
    Evaluated<SavingThrowType> {
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

