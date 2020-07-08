package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character

data class KnowLanguageStatus(
    @JsonProperty("know language")
    val name: String
): Status() {
    override fun applyStatus(c: Character) {
        TODO("Not yet implemented")
    }
}
