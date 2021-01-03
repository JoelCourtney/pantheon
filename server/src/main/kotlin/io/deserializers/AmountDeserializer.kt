package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import io.JacksonWrapper
import model.access.Expression
import model.quantities.amounts.Amount

/**
 * Deserializes objects of the [Amount] hierarchy from YAML.
 *
 * Use through [JacksonWrapper.readFile] only.
 */
object AmountDeserializer : StdDeserializer<Expression<Amount>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<Amount> {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseAmount(s)
    }
}
