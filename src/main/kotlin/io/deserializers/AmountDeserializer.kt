package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.amounts.Amount
import io.JacksonWrapper
import model.identity.Expression

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
