package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.damage.DamageUnit
import io.JacksonWrapper

/**
 * Deserializes [DamageUnit] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DamageUnitDeserializer : StdDeserializer<DamageUnit>(DamageUnit::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): DamageUnit {
        return ANTLRWrapper.parseDamageUnit(p!!.readValueAs(String::class.java))
    }
}
