package model

import com.fasterxml.jackson.annotation.JsonProperty

data class Language(
    val name: String,

    @JsonProperty("typical speakers")
    val typicalSpeakers: String,

    val script: String
)