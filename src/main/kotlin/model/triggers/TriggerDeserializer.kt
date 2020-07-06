package model.triggers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.triggers.Trigger.*
import java.lang.IllegalArgumentException

class TriggerDeserializer : StdDeserializer<Trigger>(Trigger::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Trigger {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "take damage type" -> TakeDamageTypeTrigger::class.java
            "deal damage type" -> DealDamageTypeTrigger::class.java
            else -> throw IllegalArgumentException("Unrecognized trigger key.")
        }
        val parser = tn.traverse()
        parser.codec = IOWrapper.mapper
        parser.nextToken()
        val result = ctxt!!.readValue(parser, targetClass)
        return result
    }
}