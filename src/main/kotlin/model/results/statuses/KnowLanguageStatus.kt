package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.Language

data class KnowLanguageStatus(
    @JsonProperty("know language")
    val language: String
): Status() {
    override fun applyStatus(c: Character) {
        TODO("Not yet implemented")
    }
}
