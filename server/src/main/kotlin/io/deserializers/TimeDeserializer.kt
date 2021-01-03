package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import io.JacksonWrapper
import model.access.Expression
import model.quantities.Quantity
import model.quantities.QuantityType

/**
 * Deserializes [Time] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object TimeDeserializer : StdDeserializer<Expression<Quantity<QuantityType.Time>>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<Quantity<QuantityType.Time>> {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseTime(s)
    }
}
