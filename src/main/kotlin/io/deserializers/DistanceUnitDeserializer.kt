package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.distance.DistanceUnit
import io.JacksonWrapper

/**
 * Deserializes [DistanceUnit] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DistanceUnitDeserializer : StdDeserializer<DistanceUnit>(DistanceUnit::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): DistanceUnit {
        return ANTLRWrapper.parseDistanceUnit(p!!.readValueAs(String::class.java))
    }
}
