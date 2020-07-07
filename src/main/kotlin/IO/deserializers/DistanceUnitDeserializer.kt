package IO.deserializers

import IO.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.distance.DistanceUnit

class DistanceUnitDeserializer : StdDeserializer<DistanceUnit>(DistanceUnit::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): DistanceUnit {
        return ANTLRWrapper.parseDistanceUnit(p!!.readValueAs(String::class.java))
    }
}