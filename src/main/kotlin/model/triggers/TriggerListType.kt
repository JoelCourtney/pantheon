package model.triggers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.conditions.ConditionListType

enum class TriggerListType {
    ANY,
    ALL
}

class TriggerListTypeDeserializer : StdDeserializer<TriggerListType>(TriggerListType::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): TriggerListType {
        return TriggerListType.valueOf(IOWrapper.enumString(p))
    }
}