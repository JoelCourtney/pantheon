package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.distance.Distance
import io.JacksonWrapper

/**
 * Deserializes objects of the [Distance] hierarchy from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DistanceDeserializer : StdDeserializer<Distance>(Distance::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Distance {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseDistance(s)
    }
}
