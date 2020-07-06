package model.triggers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

class TriggerDeserializer : StdDeserializer<Trigger>(Trigger::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Trigger {
        TODO("not yet bro")
    }
}