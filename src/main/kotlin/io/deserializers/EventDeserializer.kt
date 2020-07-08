package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.logic.Event
import io.JacksonWrapper

/**
 * Deserializes [Event] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object EventDeserializer : StdDeserializer<Event>(Event::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Event {
        val s = p!!.readValueAs(String::class.java)
        return Event(s)
    }
}
