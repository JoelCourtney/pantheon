package io.deserializers

import io.JacksonWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.results.*
import model.results.effects.ModifyOnceEffect
import model.results.effects.SpawnEventEffect
import model.results.statuses.ModifyStatus
import model.results.statuses.SetStatus
import java.lang.IllegalArgumentException

/**
 * Deserializes all implementors of [Result] from YAML.
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
 * - if | if not            -> [ConditionalResult]
 * - know language          -> [KnowLanguageStatus]
 * - saving throw           -> [SavingThrowResult]
 * - spawn effect           -> [SpawnEventEffect]
 * - take damage            -> [TakeDamageEffect]
 * - until | not until      -> [TimedResult]
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
class ResultDeserializer : StdDeserializer<Result>(Result::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Result {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "if", "if not" -> ConditionalResult::class.java
            "saving throw" -> SavingThrowResult::class.java
            "spawn event" -> SpawnEventEffect::class.java
            "until", "not until" -> TimedResult::class.java
            "choose" -> ChooseResult::class.java
            "modify" -> ModifyStatus::class.java
            "modify once" -> ModifyOnceEffect::class.java
            "set" -> SetStatus::class.java
            "for each" -> ForEachResult::class.java
            else -> throw IllegalArgumentException("Unrecognized result key: ${keys[0]}")
        }
        val parser = tn.traverse()
        parser.codec = JacksonWrapper.mapper
        parser.nextToken()
        return ctxt!!.readValue(parser, targetClass)
    }
}
