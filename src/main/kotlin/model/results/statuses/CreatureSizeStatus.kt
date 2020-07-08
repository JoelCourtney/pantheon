package model.results.statuses

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.CreatureSize

class CreatureSizeStatus(
    @JsonProperty("creature size")
    val size: CreatureSize
) : Status() {
    override fun applyStatus(c: Character) {
        TODO("Not yet implemented")
    }
}