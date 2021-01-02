package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.JacksonWrapper
import model.access.Expression
import model.quantities.Quantity
import model.quantities.QuantityType

/**
 * Deserializes objects of the [Damage] hierarchy from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DamageDeserializer : StdDeserializer<Expression<Quantity<QuantityType.Damage>>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<Quantity<QuantityType.Damage>> {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseDamage(s)
    }
}
