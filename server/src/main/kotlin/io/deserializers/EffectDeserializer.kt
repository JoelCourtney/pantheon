package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.JacksonWrapper
import model.effects.*

class EffectDeserializer : StdDeserializer<Effect>(Effect::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Effect {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "if", "if not" -> ConditionalEffect::class.java
            "choose" -> ChoiceEffect::class.java
            "modify" -> ModifyEffect::class.java
            "for each" -> ForEachEffect::class.java
            "proficiency" -> ProficiencyEffect::class.java
            "feature" -> FeatureEffect::class.java
            "asi or feat" -> ASIOrFeatEffect::class.java
            "subclass" -> SubclassEffect::class.java
            else -> throw IllegalArgumentException("Unrecognized result key: ${keys[0]}")
        }
        val parser = tn.traverse()
        parser.codec = JacksonWrapper.mapper
        parser.nextToken()
        return ctxt!!.readValue(parser, targetClass)
    }
}
