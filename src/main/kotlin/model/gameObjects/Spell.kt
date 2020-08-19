package model.gameObjects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DistanceDeserializer
import model.access.Expression
import model.results.Result
import model.access.Evaluated
import model.quantities.QuantityType.*
import io.deserializers.TimeDeserializer
import model.quantities.Quantity

data class Spell(
    @JsonProperty("name")
    val identity: String,

    val level: Int,
    val school: SpellSchool,

    @JsonProperty("casting time")
    @JsonDeserialize(using = TimeDeserializer::class)
    val castingTime: Expression<Quantity<Time>>,
    val ritual: Boolean,

    @JsonDeserialize(using = DistanceDeserializer::class)
    val range: Expression<Quantity<Distance>>,

    val verbal: Boolean,
    val somatic: Boolean,
    val material: Boolean,

    @JsonDeserialize(using = TimeDeserializer::class)
    val duration: Expression<Quantity<Time>>,
    val concentration: Boolean,

    val description: String,

    @JsonProperty("display in combat")
    val displayInCombat: Boolean,
    @JsonProperty("display in roleplay")
    val displayInRoleplay: Boolean

): Evaluated<Spell> {
    @JsonProperty("available to")
    val availableTo: Array<String> = arrayOf()
    @JsonProperty("material description")
    val materialDescription: String = ""

    val results: Array<Result> = arrayOf()

    @JsonProperty("reaction to")
    val reactionTo: String = ""
}

