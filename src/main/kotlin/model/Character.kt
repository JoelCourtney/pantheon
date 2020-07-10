package model

import com.fasterxml.jackson.annotation.JsonProperty
import model.identity.Evaluated
import model.races.Race
import model.results.Result

/**
 * exposed properties:
 * 
 * - identity: String
 * - race: Race
 * - knownSpells: MutableList<Spell>
 * - strength: Int
 * - dexterity: Int
 * - constitution: Int
 * - intelligence: Int
 * - wisdom: Int
 * - charisma: Int
 * - size: CreatureSize
 * - knownLanguages: MutableList<Language>
 * - items: MutableList<Item>
 */
data class Character(
    @JsonProperty("name")
    val identity: String,
    val race: Race
//    val classes: Array<CharacterClass>,
): Evaluated<Character> {
    private val results: ArrayList<Result> = arrayListOf()

    fun addResult(r: Result) {
        results.add(r)
    }

    private fun processResults() {
        var i = 0
        while (i < results.size) {
            results[i].apply(this)
            i++
        }
        results.removeAll { it.purge() }
    }
}
