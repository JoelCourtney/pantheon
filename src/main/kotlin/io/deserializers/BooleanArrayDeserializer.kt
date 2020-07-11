package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import model.access.Expression

class BooleanArrayDeserializer: StdDeserializer<Array<Expression<Boolean>>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Array<Expression<Boolean>> {
        return p!!.readValueAs(Array<String>::class.java).map { ANTLRWrapper.parseBoolean(it) }.toTypedArray()
    }
}
