package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.logic.Event

class EventDeserializer : StdDeserializer<Event>(Event::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Event {
        val s = p!!.readValueAs(String::class.java)
        return Event(s)
    }
}