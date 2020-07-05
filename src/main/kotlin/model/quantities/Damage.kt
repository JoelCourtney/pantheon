package model.quantities

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

interface Damage : Quantity

data class DamageComponent(val exp: Expression, val unit: DamageUnit): Damage {
    override fun toString(): String {
        return exp.toString() + " " + unit.symbol
    }
}

enum class DamageKeyword(private val symbol: String) : Damage {
    NONE("none");

    override fun toString(): String {
        return symbol
    }
}

enum class DamageUnit(override var symbol: String) : Unit {
    ACID("acid"),
    BLUDGEONING("bludgeoning"),
    COLD("cold"),
    FIRE("fire"),
    FORCE("force"),
    LIGHTNING("lightning"),
    NECROTIC("necrotic"),
    PIERCING("piercing"),
    POISON("poison"),
    PSYCHIC("psychic"),
    RADIANT("radiant"),
    SLASHING("slashing"),
    THUNDER("thunder");
}

class DamageDeserializer : StdDeserializer<Damage>(Damage::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Damage {
        val s = p!!.readValueAs(String::class.java)
        return ParseWrapper.parseDamage(s)
    }
}