package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.time.TimeUnit

class TimeUnitDeserializer : StdDeserializer<TimeUnit>(TimeUnit::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): TimeUnit {
        return ANTLRWrapper.parseTimeUnit(p!!.readValueAs(String::class.java))
    }
}