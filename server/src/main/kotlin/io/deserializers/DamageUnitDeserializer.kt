package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import io.JacksonWrapper
import model.access.Expression
import model.quantities.DamageUnit
import model.quantities.QuantityType.Damage
import model.quantities.QuantityUnit

/**
 * Deserializes [DamageUnit] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object DamageUnitDeserializer : StdDeserializer<Expression<QuantityUnit<Damage>>>(
    Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<QuantityUnit<Damage>> {
        return ANTLRWrapper.parseDamageUnit(p!!.readValueAs(String::class.java))
    }
}
