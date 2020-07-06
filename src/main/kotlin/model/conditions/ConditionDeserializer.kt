package model.conditions

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.conditions.Condition.*
import java.lang.IllegalArgumentException

class ConditionDeserializer : StdDeserializer<Condition>(Condition::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Condition {
        val text = p!!.readValueAs(String::class.java)
        return when (text) {
            "effected by this spell" -> EffectedByThisSpellCondition()
            else -> throw IllegalArgumentException("Unrecognized condition.")
        }
    }
}