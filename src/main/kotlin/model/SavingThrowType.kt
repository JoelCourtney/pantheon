package model

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer

enum class SavingThrowType {
    STRENGTH,
    DEXTERITY,
    CONSTITUTION,
    INTELLIGENCE,
    WISDOM,
    CHARISMA,
    DEATH;
}

class SavingThrowTypeDeserializer : StdDeserializer<SavingThrowType>(SavingThrowType::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): SavingThrowType {
        val s = p!!.readValueAs(String::class.java)
        return SavingThrowType.valueOf(s.toUpperCase())
    }
}