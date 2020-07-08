package io.deserializers

import io.JacksonWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import model.results.ConditionalResult
import model.results.Result
import model.results.SavingThrowResult
import model.results.TimedResult
import model.results.effects.DealExtraDamageEffect
import model.results.effects.SpawnEventEffect
import model.results.effects.TakeDamageEffect
import model.results.statuses.*
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
object ResultDeserializer : StdDeserializer<Result>(Result::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Result {
        val tn = p!!.readValueAsTree<TreeNode>()
        val keys = tn.fieldNames().asSequence().toList()
        val targetClass = when (keys[0].toLowerCase()) {
            "ability score increase" -> AbilityScoreIncreaseStatus::class.java
            "base movement speed" -> BaseSpeedStatus::class.java
            "creature size" -> CreatureSizeStatus::class.java
            "deal extra damage" -> DealExtraDamageEffect::class.java
            "have melee weapon" -> MeleeWeaponStatus::class.java
            "have resistance" -> ResistanceStatus::class.java
            "if", "if not" -> ConditionalResult::class.java
            "know language" -> KnowLanguageStatus::class.java
            "saving throw" -> SavingThrowResult::class.java
            "spawn event" -> SpawnEventEffect::class.java
            "take damage" -> TakeDamageEffect::class.java
            "until", "not until" -> TimedResult::class.java
            else -> throw IllegalArgumentException("Unrecognized result key: ${keys[0]}")
        }
        val parser = tn.traverse()
        parser.codec = JacksonWrapper.mapper
        parser.nextToken()
        return ctxt!!.readValue(parser, targetClass)
    }
}
