package model.timers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.timers.Timer.*
import java.lang.IllegalArgumentException

class TimerDeserializer : StdDeserializer<Timer>(Timer::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Timer {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "start of turn" -> StartOfTurn::class.java
            "end of turn" -> EndOfTurn::class.java
            else -> throw IllegalArgumentException("Unrecognized timer key.")
        }
        val parser = tn.traverse()
        parser.codec = IOWrapper.mapper
        parser.nextToken()
        val result = ctxt!!.readValue(parser, targetClass)
        return result
    }
}