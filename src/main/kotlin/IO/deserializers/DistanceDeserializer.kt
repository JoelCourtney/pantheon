package IO.deserializers

import IO.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.distance.Distance

class DistanceDeserializer : StdDeserializer<Distance>(Distance::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Distance {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseDistance(s)
    }
}