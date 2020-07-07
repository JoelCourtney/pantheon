package model.races

import com.fasterxml.jackson.annotation.JsonProperty

data class RaceVariant(
    @JsonProperty("variant of")
    val variantOf: Race
) : Race() {

}