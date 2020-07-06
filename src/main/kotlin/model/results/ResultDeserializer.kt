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
        if (keys.size != 1) {
            throw IllegalArgumentException("Only one key/value pair per result is allowed.")
        }
        val key = keys[0]
        val value = (tn.get(key) as ValueNode).asText()
        return when (key) {
            "damage" -> TakeDamageResult(ParseWrapper.parseDamage(value))
            "do" -> when (value) {
                "nothing" -> NoOpResult
                else -> throw IllegalArgumentException("Unrecognized do command.")
            }
            else -> throw IllegalArgumentException("Unrecognized result key.")
        }
    }
}