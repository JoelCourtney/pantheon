package model.gameObjects

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DistanceDeserializer
import model.access.Expression
import model.quantities.QuantityType.*
import io.deserializers.TimeDeserializer
import model.access.Environment
import model.effects.Effect
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

