package model.gameObjects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DistanceDeserializer
import io.deserializers.TimeDeserializer
import model.effects.Effect
import model.gameObjects.prototypes.Prototype
import model.quantities.Distance
import model.quantities.Time

data class Spell(
    @JsonProperty("name")
    val identity: String,

    val level: Int,
    val school: SpellSchool,

    @JsonProperty("casting time")
    @JsonDeserialize(using = TimeDeserializer::class)
    val castingTime: Time,
    val ritual: Boolean,

    @JsonDeserialize(using = DistanceDeserializer::class)
    val range: Distance,

    val verbal: Boolean,
    val somatic: Boolean,
    val material: Boolean,

    @JsonDeserialize(using = TimeDeserializer::class)
    val duration: Time,
    val concentration: Boolean,

    val description: String,

    @JsonProperty("display in combat")
    val displayInCombat: Boolean,
    @JsonProperty("display in roleplay")
    val displayInRoleplay: Boolean,

    @JsonProperty("available to")
    val availableTo: Array<String>?,
    @JsonProperty("material description")
    val materialDescription: String?,
    @JsonProperty("reaction to")
    val reactionTo: String?
): Prototype(standardEffects()) {

    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }

    companion object {
        fun standardEffects() = listOf<Effect>()
    }
}

