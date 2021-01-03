package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import io.JacksonWrapper
import model.access.Expression
import model.quantities.QuantityType.Time
import model.quantities.QuantityUnit

/**
 * Deserializes [TimeUnit] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object TimeUnitDeserializer : StdDeserializer<Expression<QuantityUnit<Time>>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<QuantityUnit<Time>> {
        return ANTLRWrapper.parseTimeUnit(p!!.readValueAs(String::class.java))
    }
}
