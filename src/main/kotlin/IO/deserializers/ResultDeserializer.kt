package IO.deserializers

import IO.JacksonWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.results.ConditionalResult
import model.results.Result
import model.results.SavingThrowResult
import model.results.TimedResult
import model.results.effects.DealExtraDamageEffect
import model.results.effects.GainResistanceEffect
import model.results.effects.TakeDamageEffect
import java.lang.IllegalArgumentException

class ResultDeserializer : StdDeserializer<Result>(Result::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Result {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "until", "not until" -> TimedResult::class.java
            "if", "not if" -> ConditionalResult::class.java
            "saving throw" -> SavingThrowResult::class.java
            "take damage" -> TakeDamageEffect::class.java
            "deal extra damage" -> DealExtraDamageEffect::class.java
            "gain resistance" -> GainResistanceEffect::class.java
            else -> throw IllegalArgumentException("Unrecognized result key.")
        }
        val parser = tn.traverse()
        parser.codec = JacksonWrapper.mapper
        parser.nextToken()
        return ctxt!!.readValue(parser, targetClass)
    }
}