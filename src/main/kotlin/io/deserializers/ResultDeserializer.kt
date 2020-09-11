package io.deserializers

import io.JacksonWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.modifications.*
import model.modifications.results.ModifyOnceResult
import model.modifications.results.SpawnEventResult
import model.modifications.effects.ModifyEffect
import model.modifications.effects.SetEffect
import model.modifications.results.SavingThrowResult
import model.modifications.results.TimedResult
import java.lang.IllegalArgumentException

/**
 * Deserializes all implementors of [Modification] from YAML.
 *
 * Chooses the subclass to deserialize as based on the first key.  
 * E.G. if "saving throw" is the first key, it will be parsed as [SavingThrowResult].
 *
 * ## List of parsing decisions (key -> class):
 * 
 * - ability score increase -> [AbilityScoreIncreaseStatus]
 * - base movement speed    -> [BaseSpeedStatus]
 * - creature size          -> [CreatureSizeStatus]
 * - deal extra damage      -> [DealExtraDamageEffect]
 * - have melee weapon      -> [MeleeWeaponStatus]
 * - have resistance        -> [ResistanceStatus]
 * - if | if not            -> [ConditionalLogic]
 * - know language          -> [KnowLanguageStatus]
 * - saving throw           -> [SavingThrowResult]
 * - spawn effect           -> [SpawnEventResult]
 * - take damage            -> [TakeDamageEffect]
 * - until | not until      -> [TimedResult]
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
class ResultDeserializer : StdDeserializer<Modification>(Modification::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Modification {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "if", "if not" -> ConditionalLogic::class.java
            "saving throw" -> SavingThrowResult::class.java
            "spawn event" -> SpawnEventResult::class.java
            "until", "not until" -> TimedResult::class.java
            "choose" -> ChoiceLogic::class.java
            "modify" -> ModifyEffect::class.java
            "modify once" -> ModifyOnceResult::class.java
            "set" -> SetEffect::class.java
            "for each" -> ForEachLogic::class.java
            else -> throw IllegalArgumentException("Unrecognized result key: ${keys[0]}")
        }
        val parser = tn.traverse()
        parser.codec = JacksonWrapper.mapper
        parser.nextToken()
        return ctxt!!.readValue(parser, targetClass)
    }
}
