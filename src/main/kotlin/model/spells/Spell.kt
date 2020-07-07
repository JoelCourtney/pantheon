package model.spells

import com.fasterxml.jackson.annotation.JsonProperty
import model.quantities.distance.Distance
import model.quantities.time.Time
import model.results.Result

data class Spell(
    val name: String,

    val level: Int,
    val school: SpellSchool,

    @JsonProperty("casting time")
    val castingTime: Time,
    val ritual: Boolean,

    val range: Distance,

    val verbal: Boolean,
    val somatic: Boolean,
    val material: Boolean,

    val duration: Time,
    val concentration: Boolean,

    val description: String,

    @JsonProperty("display in combat")
    val displayInCombat: Boolean,
    @JsonProperty("display in roleplay")
    val displayInRoleplay: Boolean

) {
    @JsonProperty("available to")
    val availableTo: Array<String> = arrayOf()
    @JsonProperty("material description")
    val materialDescription: String = ""

    val results: Array<Result> = arrayOf()

    @JsonProperty("reaction to")
    val reactionTo: String = ""
}

