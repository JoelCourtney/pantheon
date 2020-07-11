package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import model.access.Expression
import model.results.SavingThrowType

class SavingThrowTypeDeserializer: StdDeserializer<Expression<SavingThrowType>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<SavingThrowType> {
        return ANTLRWrapper.parseSavingThrowType(p!!.readValueAs(String::class.java))
    }
}
