package io.deserializers

import io.JacksonWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.effects.*
import java.lang.IllegalArgumentException

class EffectDeserializer : StdDeserializer<Effect>(Effect::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Effect {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "if", "if not" -> ConditionalEffect::class.java
            "choose" -> ChoiceEffect::class.java
            "modify" -> ModifyEffect::class.java
            "set" -> SetEffect::class.java
            "for each" -> ForEachEffect::class.java
            else -> throw IllegalArgumentException("Unrecognized result key: ${keys[0]}")
        }
        val parser = tn.traverse()
        parser.codec = JacksonWrapper.mapper
        parser.nextToken()
        return ctxt!!.readValue(parser, targetClass)
    }
}
