package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.time.Time
import io.JacksonWrapper

/**
 * Deserializes [Time] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object TimeDeserializer : StdDeserializer<Time>(Time::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Time {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseTime(s)
    }
}
