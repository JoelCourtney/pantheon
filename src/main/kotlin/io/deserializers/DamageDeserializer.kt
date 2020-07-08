package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.damage.Damage
import io.JacksonWrapper

/**
 * Deserializes objects of the [Damage] hierarchy from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DamageDeserializer : StdDeserializer<Damage>(Damage::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Damage {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseDamage(s)
    }
}
