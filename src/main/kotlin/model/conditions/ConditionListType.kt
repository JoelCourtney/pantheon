package model.conditions

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

enum class ConditionListType {
    ANY,
    ALL
}

class ConditionListTypeDeserializer : StdDeserializer<ConditionListType>(ConditionListType::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): ConditionListType {
        val s = p!!.readValueAs(String::class.java)
        return ConditionListType.valueOf(s.toUpperCase())
    }
}