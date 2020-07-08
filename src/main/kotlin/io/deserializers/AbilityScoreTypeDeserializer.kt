package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.AbilityScoreType

class AbilityScoreTypeDeserializer : StdDeserializer<AbilityScoreType>(AbilityScoreType::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): AbilityScoreType {
        return AbilityScoreType.fromString(p!!.readValueAs(String::class.java))
    }
}