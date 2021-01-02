package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.JacksonWrapper
import model.access.Expression
import model.quantities.DistanceUnit
import model.quantities.QuantityUnit
import model.quantities.QuantityType.Distance

/**
 * Deserializes [DistanceUnit] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DistanceUnitDeserializer : StdDeserializer<Expression<QuantityUnit<Distance>>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<QuantityUnit<Distance>> {
        return ANTLRWrapper.parseDistanceUnit(p!!.readValueAs(String::class.java))
    }
}
