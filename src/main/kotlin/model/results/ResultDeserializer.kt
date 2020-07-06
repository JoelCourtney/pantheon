package model.results

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import com.fasterxml.jackson.databind.node.ValueNode
import java.lang.IllegalArgumentException

class ResultDeserializer : StdDeserializer<Result>(Result::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Result {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "triggers" -> TriggeredResult::class.java
            "conditions" -> ConditionedResult::class.java
            "timers" -> TimedResult::class.java
            "saving throw" -> SavingThrowResult::class.java
            "take damage" -> TakeDamageResult::class.java
            "deal extra damage" -> DealExtraDamageResult::class.java
            else -> throw IllegalArgumentException("Unrecognized result key.")
        }
        val parser = tn.traverse()
        parser.codec = IOWrapper.mapper
        parser.nextToken()
        val result = ctxt!!.readValue(parser, targetClass)
        return result
    }
}