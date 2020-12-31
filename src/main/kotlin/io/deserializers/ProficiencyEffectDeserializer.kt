package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import model.effects.ProficiencyEffect
import model.gameObjects.Proficiency
import model.gameObjects.ProficiencyMultiplier
import model.gameObjects.SavingThrow
import model.gameObjects.Skill

class ProficiencyEffectDeserializer: StdDeserializer<ProficiencyEffect>(ProficiencyEffect::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): ProficiencyEffect {
        val tn = p!!.readValueAsTree<TreeNode>()
        val name = ANTLRWrapper.parseStringExpression(tn["proficiency"].toString().drop(1).dropLast(1))
        val multiplier = if (tn.fieldNames().asSequence().contains("double")) {
            ProficiencyMultiplier.fromDoubleBoolean(tn["double"].toString().drop(1).dropLast(1).toBoolean())
        } else {
            ProficiencyMultiplier.SINGLE
        }
        return ProficiencyEffect(when (tn["type"].toString().drop(1).dropLast(1).toLowerCase()) {
            "armor" -> Proficiency.ArmorProficiency(name)
            "weapon" -> Proficiency.WeaponProficiency(name)
            "language" -> Proficiency.LanguageProficiency(name)
            "tool" -> Proficiency.ToolProficiency(name, multiplier)
            "skill" -> Proficiency.SkillProficiency(Skill.fromStringExpression(name), multiplier)
            "saving throw" -> Proficiency.SavingThrowProficiency(SavingThrow.fromStringExpression(name))
            else -> TODO("make an error for this")
        })
    }
}