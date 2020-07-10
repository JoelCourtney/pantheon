package model

import com.fasterxml.jackson.annotation.JsonProperty
import model.identity.Evaluated

data class Language(
    @JsonProperty("name")
    val identity: String,

    @JsonProperty("typical speakers")
    val typicalSpeakers: String,

    val script: String
): Evaluated<Language>
