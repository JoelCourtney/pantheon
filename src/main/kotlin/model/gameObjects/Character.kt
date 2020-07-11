package model.gameObjects

import com.fasterxml.jackson.annotation.JsonProperty
import model.CreatureSize
import model.access.Evaluated
import model.results.Result
import model.spells.Spell

/**
 * exposed properties:
 * 
 * - identity: String
 * - race: Race
 * - known spells: MutableList<Spell>
 * - strength: Int
 * - dexterity: Int
 * - constitution: Int
 * - intelligence: Int
 * - wisdom: Int
 * - charisma: Int
 * - size: CreatureSize
 * - known languages: MutableList<Language>
 * - items: MutableList<Item>
 */
data class Character(
    @JsonProperty("name")
    var identity: String,
    var race: Race,
//    var classes: MutableList<Class>,
    var strength: Int,
    var dexterity: Int,
    var constitution: Int,
    var wisdom: Int,
    var charisma: Int,
    var health: Int,
    var tempHealth: Int,
    var items: MutableList<Item>,
//    var background: Background,
    var inspired: Boolean
//    var statuses: MutableList<Status>
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
