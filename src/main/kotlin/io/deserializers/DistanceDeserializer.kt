package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.access.Expression
import io.JacksonWrapper
import model.quantities.Quantity
import model.quantities.QuantityType

/**
 * Deserializes objects of the [Distance] hierarchy from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DistanceDeserializer : StdDeserializer<Expression<Quantity<QuantityType.Distance>>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<Quantity<QuantityType.Distance>> {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseDistance(s)
    }
}
