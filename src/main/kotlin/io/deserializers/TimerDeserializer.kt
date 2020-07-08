package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import io.JacksonWrapper
import model.logic.Timer

/**
 * Deserializes [Timer] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object TimerDeserializer : StdDeserializer<Timer>(Timer::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Timer {
        val s = p!!.readValueAs(String::class.java)
        return Timer(ANTLRWrapper.parseTime(s))
    }
}
