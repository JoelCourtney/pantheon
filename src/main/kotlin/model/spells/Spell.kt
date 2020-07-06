package model.spells

import model.quantities.*
import model.results.Result

data class Spell(
    val name: String,

    val level: Int,
    val school: SpellSchool,

    val `casting time`: Time,
    val ritual: Boolean,

    val range: Distance,

    val verbal: Boolean,
    val somatic: Boolean,
    val material: Boolean,

    val duration: Time,
    val concentration: Boolean,

    val description: String,

    val `display in combat`: Boolean,
    val `display in roleplay`: Boolean

) {
    val `available to`: Array<String> = arrayOf()
    val `material description`: String = ""

    val results: Array<Result> = arrayOf()

    val `reaction to`: String = ""
}

