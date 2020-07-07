package IO.deserializers

import IO.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.quantities.amounts.Amount

class AmountDeserializer : StdDeserializer<Amount>(Amount::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Amount {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseAmount(s)
    }
}